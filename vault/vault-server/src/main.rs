extern crate core;
#[macro_use]
extern crate rocket;

use mongodb::{bson, Client, Collection, Database};
use rocket::futures::StreamExt;
use rocket::serde::json::Json;
use rocket::State;
use tracing_subscriber;
use tracing_subscriber::FmtSubscriber;

use db::VaultDoc;

use crate::api::{
    EncryptedMessage, JoinRequest, RegistrationResponse, RegistrationStatus, UserSignature,
    VaultInfo, VaultInfoStatus,
};
use crate::db::{Db, DbSchema, SecretDistributionDoc};

mod db;
mod crypto;
mod api;


/// Register a new distributed vault
/// Registration example:
/// first device: curl -X POST http://localhost:8000/register -H 'Content-Type: application/json' -d '{"vaultName":"test_vault","publicKey":"ZE+rI1+X7IsWkCbnTamDtfvvavrIp7UfAtpUVJXfBZ8=","signature":"OOshi5j4XmhxJfCtd3DiQkPIe87NxEc5TvSkqlma+0qxAEWKBpvy4HCR+yKll5p8R1ttKKL9UG9IO2rIIxm6DQ=="}'
/// second device: curl -X POST http://localhost:8000/register -H 'Content-Type: application/json' -d '{"vaultName":"test_vault","publicKey":"Mi6MUjlvim7r2Qz5Ug63ZnkXhaDoBWh3os/ItPzP3Aw=","signature":"haE9QJfSZyLYuKOP9dao0gI2i/bCnjFh6Zph72xgpftuTdzAOotnB5D8r8+IsPFWhqEIpKzEBGsrA59H433xBw=="}'
///
#[post("/register", format = "json", data = "<register_request>")]
async fn register(register_request: Json<UserSignature>, db: &State<Db>) -> Json<RegistrationResponse> {
    info!("Register a new vault or join");

    let vaults_col = db.vaults_col();

    let vault_request= register_request.into_inner();

    info!("verify: {:?}", vault_request);
    let is_valid = crypto::verify(&vault_request);

    if !is_valid {
        panic!("Can't pass signature verification");
    }

    //find user
    let vault_filter = bson::doc! {
        "vaultName": vault_request.vault_name.clone()
    };

    let maybe_vault: Option<VaultDoc> = vaults_col
        .find_one(vault_filter, None)
        .await
        .unwrap();

    return match maybe_vault {
        None => {
            //create a new user:
            vaults_col.insert_one(vault_request.to_initial_vault_doc(), None)
                .await
                .unwrap();

            Json(RegistrationResponse {
                status: RegistrationStatus::Registered,
                result: "Vault has been created".to_string(),
            })
        }
        Some(mut vault_doc) => {
            //if vault already exists
            if vault_doc.signatures.contains(&vault_request) {
                return Json(RegistrationResponse {
                    status: RegistrationStatus::Registered,
                    result: "Vault already exists and you are one of the members".to_string(),
                });
            }

            vault_doc.pending_joins.push(vault_request.clone());

            let vault_name = vault_request.vault_name.clone();
            let vault_filter = bson::doc! {
                "vaultName": vault_name
            };
            vaults_col.replace_one(vault_filter, vault_doc.clone(), None)
                .await
                .unwrap();

            Json(RegistrationResponse {
                status: RegistrationStatus::AlreadyExists,
                result: "Added to pending requests".to_string(),
            })
        }
    };
}

#[post("/decline", format = "json", data = "<join_request>")]
async fn decline(db: &State<Db>, join_request: Json<JoinRequest>) -> Json<String> {
    let join_request = join_request.into_inner();
    info!("Decline join request");

    let vaults_col = db.vaults_col();

    let vaults_filter = bson::doc! {
        "vaultName": join_request.member.vault_name.clone()
    };
    let maybe_vault: Option<VaultDoc> = vaults_col
        .find_one(vaults_filter, None)
        .await.unwrap();

    let vault_name = join_request.candidate.clone().vault_name;
    let candidate = join_request.candidate;

    return match maybe_vault {
        //user not found
        None => {
            panic!("Vault not found!");
        }
        Some(mut vault_doc) => {
            if vault_doc.signatures.contains(&candidate) {
                remove_candidate_from_pending_queue(&candidate, &mut vault_doc);
                update_vault(vault_name.clone(), vaults_col, vault_doc).await;
                return Json("Candidate is already a member of the vault".to_string());
            }

            if vault_doc.signatures.contains(&join_request.member) {
                if vault_doc.pending_joins.contains(&candidate) {
                    if crypto::verify(&candidate) {
                        //we can add a new user signature into a vault
                        remove_candidate_from_pending_queue(&candidate, &mut vault_doc);
                        vault_doc.declined_joins.push(candidate.clone());
                        update_vault(vault_name.clone(), vaults_col, vault_doc).await;
                    }
                }
            }

            Json(String::from("Success"))
        }
    };
}

/// Accept join request
/// example:
/// curl -X POST http://localhost:8000/accept -H 'Content-Type: application/json' -d '{"member": {"vaultName":"test_vault","publicKey":"ZE+rI1+X7IsWkCbnTamDtfvvavrIp7UfAtpUVJXfBZ8=","signature":"OOshi5j4XmhxJfCtd3DiQkPIe87NxEc5TvSkqlma+0qxAEWKBpvy4HCR+yKll5p8R1ttKKL9UG9IO2rIIxm6DQ=="}, "candidate": {"vaultName":"test_vault","publicKey":"Mi6MUjlvim7r2Qz5Ug63ZnkXhaDoBWh3os/ItPzP3Aw=","signature":"haE9QJfSZyLYuKOP9dao0gI2i/bCnjFh6Zph72xgpftuTdzAOotnB5D8r8+IsPFWhqEIpKzEBGsrA59H433xBw=="}}'
#[post("/accept", format = "json", data = "<join_request>")]
async fn accept(db: &State<Db>, join_request: Json<JoinRequest>) -> Json<String> {
    let join_request = join_request.into_inner();
    info!("Accept join request");

    let vaults_col = db.vaults_col();

    let vaults_filter = bson::doc! {
        "vaultName": join_request.member.vault_name.clone()
    };
    let maybe_vault: Option<VaultDoc> = vaults_col
        .find_one(vaults_filter, None)
        .await.unwrap();

    return match maybe_vault {
        //user not found
        None => {
            panic!("Vault not found!");
        }
        Some(mut vault_doc) => {
            let candidate = join_request.candidate;
            if vault_doc.signatures.contains(&candidate) {
                remove_candidate_from_pending_queue(&candidate, &mut vault_doc);
                update_vault(candidate.vault_name.clone(), vaults_col, vault_doc).await;
                return Json("Candidate is already a member of the vault".to_string());
            }

            if vault_doc.signatures.contains(&join_request.member) {
                if vault_doc.pending_joins.contains(&candidate) {
                    if crypto::verify(&candidate) {
                        //we can add a new user signature into a vault
                        remove_candidate_from_pending_queue(&candidate, &mut vault_doc);

                        vault_doc.signatures.push(candidate.clone());

                        update_vault(candidate.vault_name.clone(), vaults_col, vault_doc).await;
                    }
                }
            }

            Json(String::from("Successful"))
        }
    };
}

#[post("/getVault", format = "json", data = "<user_signature>")]
async fn get_vault(db: &State<Db>, user_signature: Json<UserSignature>) -> Json<VaultInfo> {
    let user_signature = user_signature.into_inner();

    let vaults_col = db.vaults_col();
    let vaults_filter = bson::doc! {
        "vaultName": user_signature.vault_name.clone()
    };

    let maybe_vault: Option<VaultDoc> = vaults_col
        .find_one(vaults_filter, None)
        .await
        .unwrap();

    return match maybe_vault {
        None => {
            Json(VaultInfo::unknown())
        }
        Some(vault) => {
            if vault.signatures.contains(&user_signature) {
                return Json(VaultInfo { status: VaultInfoStatus::Member, vault: Some(vault) });
            }

            if vault.pending_joins.contains(&user_signature) {
                return Json(VaultInfo::pending());
            }

            if vault.declined_joins.contains(&user_signature) {
                return Json(VaultInfo::declined());
            }

            Json(VaultInfo::unknown())
        }
    };
}

#[post("/distribute", format = "json", data = "<encrypted_password_share>")]
async fn distribute(db: &State<Db>, encrypted_password_share: Json<EncryptedMessage>) -> Json<String> {
    let secrets_distribution_col = db.distribution_col();

    //create a new user:
    let record = SecretDistributionDoc {
        secret_message: encrypted_password_share.into_inner()
    };

    secrets_distribution_col.insert_one(record, None)
        .await
        .unwrap();

    Json("OK".to_string())
}

#[post("/findShares", format = "json", data = "<user_signature>")]
async fn find_shares(db: &State<Db>, user_signature: Json<UserSignature>) -> Json<Vec<SecretDistributionDoc>> {
    let secrets_distribution_col = db.distribution_col();

    //find shares
    let secret_shares_filter = bson::doc! {
        "secret_message.receiver.rsa_public_key": user_signature.into_inner().rsa_public_key.clone()
    };

    let mut shares_docs = secrets_distribution_col
        .find(secret_shares_filter, None)
        .await
        .unwrap();

    let mut shares = vec![];
    while let Some(share) = shares_docs.next().await {
        shares.push(share.unwrap());
    }

    Json(shares)
}

async fn update_vault(vault_name: String, vaults_col: Collection<VaultDoc>, mut vault_doc: VaultDoc) {
    let vault_filter = bson::doc! {
        "vaultName": vault_name
    };

    vaults_col.replace_one(vault_filter, vault_doc, None)
        .await
        .unwrap();
}

fn remove_candidate_from_pending_queue(candidate: &UserSignature, vault_doc: &mut VaultDoc) {
    let maybe_index = vault_doc
        .pending_joins
        .iter()
        .position(|sig| *sig == *candidate);

    if let Some(index) = maybe_index {
        vault_doc.pending_joins
            //remove signature from pending
            .remove(index);
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let db_schema = DbSchema::default();

    let url = format!("mongodb://meta-secret-db:{}/", 27017);
    let client: Client = Client::with_uri_str(&url).await.unwrap();
    let mongo_db = client.database(db_schema.db_name.as_str());
    let db = Db {
        db_schema,
        url,
        client,
        db: mongo_db
    };

    let _rocket = rocket::build()
        .manage(db)
        .mount("/", routes![
            register, accept, decline, get_vault, distribute, find_shares
        ])
        .launch()
        .await?;

    Ok(())
}
