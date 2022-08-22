#[macro_use]
extern crate rocket;
extern crate core;

use std::borrow::Borrow;

use ed25519_dalek::{Keypair, PublicKey, Signature, Verifier};
use mongodb::{bson, Client, Collection};
use mongodb::options::FindOneAndUpdateOptions;
use rand::rngs::OsRng;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use tracing::info;
use tracing_subscriber;
use tracing_subscriber::FmtSubscriber;

use db::VaultDoc;

use crate::api::{JoinRequest, RegistrationResponse, RegistrationStatus, UserSignature};
use crate::bson::Document;
use crate::db::{DbSchema};

mod db;
mod crypto;
mod api;


/// Register a new distributed vault
/// Registration example:
/// first device: curl -X POST http://localhost:8000/register -H 'Content-Type: application/json' -d '{"vaultName":"test_vault","publicKey":"ZE+rI1+X7IsWkCbnTamDtfvvavrIp7UfAtpUVJXfBZ8=","signature":"OOshi5j4XmhxJfCtd3DiQkPIe87NxEc5TvSkqlma+0qxAEWKBpvy4HCR+yKll5p8R1ttKKL9UG9IO2rIIxm6DQ=="}'
/// second device: curl -X POST http://localhost:8000/register -H 'Content-Type: application/json' -d '{"vaultName":"test_vault","publicKey":"Mi6MUjlvim7r2Qz5Ug63ZnkXhaDoBWh3os/ItPzP3Aw=","signature":"haE9QJfSZyLYuKOP9dao0gI2i/bCnjFh6Zph72xgpftuTdzAOotnB5D8r8+IsPFWhqEIpKzEBGsrA59H433xBw=="}'
///
#[post("/register", format = "json", data = "<register_request>")]
async fn register(register_request: Json<UserSignature>) -> Json<RegistrationResponse> {
    info!("Register a new vault or join");

    let vaults_col = get_vaults_col().await;

    let vault_request = register_request.into_inner();

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
                result: "Vault has been created".to_string()
            })
        }
        Some(mut vault_doc) => {
            //if vault already exists
            if vault_doc.signatures.contains(&vault_request) {
                return Json(RegistrationResponse {
                    status: RegistrationStatus::Registered,
                    result: "Vault already exists and you are one of the members".to_string()
                })
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
                result: "Added to pending requests".to_string()
            })
        }
    };
}

#[post("/decline", format = "json", data = "<join_request>")]
async fn decline(join_request: Json<JoinRequest>) -> Json<String> {
    let join_request = join_request.into_inner();
    info!("Decline join request");

    let vaults_col = get_vaults_col().await;

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
            if vault_doc.signatures.contains(&join_request.candidate) {
                remove_candidate_from_pending_queue(&join_request, &mut vault_doc);
                update_vault(join_request, vaults_col, vault_doc).await;
                return Json("Candidate is already a member of the vault".to_string())
            }

            if vault_doc.signatures.contains(&join_request.member) {
                if vault_doc.pending_joins.contains(&join_request.candidate) {
                    if crypto::verify(&join_request.candidate) {
                        //we can add a new user signature into a vault
                        remove_candidate_from_pending_queue(&join_request, &mut vault_doc);
                        update_vault(join_request, vaults_col, vault_doc).await;
                    }
                }
            }

            Json(String::from("Success"))
        }
    }
}

/// Accept join request
/// example:
/// curl -X POST http://localhost:8000/accept -H 'Content-Type: application/json' -d '{"member": {"vaultName":"test_vault","publicKey":"ZE+rI1+X7IsWkCbnTamDtfvvavrIp7UfAtpUVJXfBZ8=","signature":"OOshi5j4XmhxJfCtd3DiQkPIe87NxEc5TvSkqlma+0qxAEWKBpvy4HCR+yKll5p8R1ttKKL9UG9IO2rIIxm6DQ=="}, "candidate": {"vaultName":"test_vault","publicKey":"Mi6MUjlvim7r2Qz5Ug63ZnkXhaDoBWh3os/ItPzP3Aw=","signature":"haE9QJfSZyLYuKOP9dao0gI2i/bCnjFh6Zph72xgpftuTdzAOotnB5D8r8+IsPFWhqEIpKzEBGsrA59H433xBw=="}}'
#[post("/accept", format = "json", data = "<join_request>")]
async fn accept(join_request: Json<JoinRequest>) -> Json<String> {
    let join_request = join_request.into_inner();
    info!("Accept join request");

    let vaults_col = get_vaults_col().await;

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
            if vault_doc.signatures.contains(&join_request.candidate) {
                remove_candidate_from_pending_queue(&join_request, &mut vault_doc);
                update_vault(join_request, vaults_col, vault_doc).await;
                return Json("Candidate is already a member of the vault".to_string())
            }

            if vault_doc.signatures.contains(&join_request.member) {
                if vault_doc.pending_joins.contains(&join_request.candidate) {
                    if crypto::verify(&join_request.candidate) {
                        //we can add a new user signature into a vault
                        remove_candidate_from_pending_queue(&join_request, &mut vault_doc);

                        vault_doc.signatures.push(join_request.candidate.clone());

                        update_vault(join_request, vaults_col, vault_doc).await;
                    }
                }
            }

            Json(String::from("Successful"))
        }
    }
}

#[post("/getVault", format = "json", data = "<user_signature>")]
async fn get_vault(user_signature: Json<UserSignature>) -> Json<VaultDoc> {
    let vaults_col = get_vaults_col().await;
    let vaults_filter = bson::doc! {
        "vaultName": user_signature.vault_name.clone()
    };

    let maybe_vault: Option<VaultDoc> = vaults_col
        .find_one(vaults_filter, None)
        .await
        .unwrap();

    return Json(maybe_vault.unwrap());
}

async fn update_vault(join_request: JoinRequest, vaults_col: Collection<VaultDoc>, mut vault_doc: VaultDoc) {
    let candidate_vault = join_request.candidate.vault_name.clone();
    let vault_filter = bson::doc! {
                            "vaultName": candidate_vault
                         };

    vaults_col.replace_one(vault_filter, vault_doc, None)
        .await
        .unwrap();
}

fn remove_candidate_from_pending_queue(join_request: &JoinRequest, vault_doc: &mut VaultDoc) {
    let maybe_index = vault_doc
        .pending_joins
        .iter()
        .position(|sig| *sig == join_request.candidate);

    if let Some(index) = maybe_index {
        vault_doc.pending_joins
            //remove signature from pending
            .remove(index);
    }
}

async fn get_vaults_col() -> Collection<VaultDoc> {
    let db_schema = DbSchema::default();

    let url = format!("mongodb://meta-secret-db:{}/", 27017);
    let client: Client = Client::with_uri_str(&url).await.unwrap();
    let db = client.database(db_schema.db_name.as_str());
    let vaults_col = db.collection::<VaultDoc>(db_schema.vault_col.as_str());
    vaults_col
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();
    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|_err| eprintln!("Unable to set global default subscriber"));
    //.expect("TODO: can't configure logger");

    let _rocket = rocket::build()
        .mount("/", routes![register, accept, get_vault])
        .launch()
        .await?;

    Ok(())
}
