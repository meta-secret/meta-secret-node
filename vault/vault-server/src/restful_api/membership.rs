use mongodb::{bson, Collection};
use rocket::serde::json::Json;
use rocket::State;

use crate::crypto::crypto;
use crate::restful_api::commons;
use crate::{Db, JoinRequest, UserSignature, VaultDoc};

#[post("/decline", format = "json", data = "<join_request>")]
pub async fn decline(db: &State<Db>, join_request: Json<JoinRequest>) -> Json<String> {
    let join_request = join_request.into_inner();
    info!("Decline join request");

    let vault_name = join_request.candidate.clone().vault_name;
    let candidate = join_request.candidate;

    let maybe_vault = commons::find_vault(db, &join_request.member).await;

    return match maybe_vault {
        //user not found
        None => {
            panic!("Vault not found!");
        }
        Some(mut vault_doc) => {
            if vault_doc.signatures.contains(&candidate) {
                remove_candidate_from_pending_queue(&candidate, &mut vault_doc);

                let vaults_col = db.vaults_col();
                update_vault(vault_name.clone(), vaults_col, vault_doc).await;
                return Json("Candidate is already a member of the vault".to_string());
            }

            if vault_doc.signatures.contains(&join_request.member) {
                if vault_doc.pending_joins.contains(&candidate) {
                    if crypto::verify(&candidate) {
                        //we can add a new user signature into a vault
                        let vaults_col = db.vaults_col();
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
pub async fn accept(db: &State<Db>, join_request: Json<JoinRequest>) -> Json<String> {
    let join_request = join_request.into_inner();
    info!("Accept join request");

    let maybe_vault = commons::find_vault(db, &join_request.member).await;

    return match maybe_vault {
        //user not found
        None => {
            panic!("Vault not found!");
        }
        Some(mut vault_doc) => {
            let candidate = join_request.candidate;
            if vault_doc.signatures.contains(&candidate) {
                remove_candidate_from_pending_queue(&candidate, &mut vault_doc);
                let vaults_col = db.vaults_col();
                update_vault(candidate.vault_name.clone(), vaults_col, vault_doc).await;
                return Json("Candidate is already a member of the vault".to_string());
            }

            if vault_doc.signatures.contains(&join_request.member) {
                if vault_doc.pending_joins.contains(&candidate) {
                    if crypto::verify(&candidate) {
                        //we can add a new user signature into a vault
                        remove_candidate_from_pending_queue(&candidate, &mut vault_doc);

                        vault_doc.signatures.push(candidate.clone());
                        let vaults_col = db.vaults_col();
                        update_vault(candidate.vault_name.clone(), vaults_col, vault_doc).await;
                    }
                }
            }

            Json(String::from("Successful"))
        }
    };
}

fn remove_candidate_from_pending_queue(candidate: &UserSignature, vault_doc: &mut VaultDoc) {
    let maybe_index = vault_doc
        .pending_joins
        .iter()
        .position(|sig| *sig == *candidate);

    if let Some(index) = maybe_index {
        vault_doc
            .pending_joins
            //remove signature from pending
            .remove(index);
    }
}

async fn update_vault(vault_name: String, vaults_col: Collection<VaultDoc>, vault_doc: VaultDoc) {
    let vault_filter = bson::doc! {
        "vaultName": vault_name
    };

    vaults_col
        .replace_one(vault_filter, vault_doc, None)
        .await
        .unwrap();
}
