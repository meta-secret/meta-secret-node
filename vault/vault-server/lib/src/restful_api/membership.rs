use crate::api::api::JoinRequest;
use mongodb::{bson, Collection};
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

use crate::api::api::UserSignature;
use crate::crypto::crypto;
use crate::db::{Db, VaultDoc};
use crate::restful_api::commons;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MembershipStatus {
    VaultNotFound,
    /// Device is a member of a vault already
    AlreadyMember,
    /// Operation finished successfully
    Finished,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MemberShipResponse {
    status: MembershipStatus,
    msg: String,
}

#[post("/decline", format = "json", data = "<join_request>")]
pub async fn decline(db: &State<Db>, join_request: Json<JoinRequest>) -> Json<MemberShipResponse> {
    let join_request = join_request.into_inner();

    let vault_name = join_request.candidate.clone().vault_name;
    let candidate = join_request.candidate;

    let maybe_vault = commons::find_vault(db, &join_request.member).await;

    return match maybe_vault {
        //user not found
        None => Json(MemberShipResponse {
            status: MembershipStatus::VaultNotFound,
            msg: "Vault not found".to_string(),
        }),
        Some(mut vault_doc) => {
            if vault_doc.signatures.contains(&candidate) {
                remove_candidate_from_pending_queue(&candidate, &mut vault_doc);

                let vaults_col = db.vaults_col();
                update_vault(vault_name.clone(), vaults_col, vault_doc).await;
                return Json(MemberShipResponse {
                    status: MembershipStatus::AlreadyMember,
                    msg: "Device is already a member of the vault".to_string(),
                });
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

            Json(MemberShipResponse {
                status: MembershipStatus::Finished,
                msg: "Device has been blocked".to_string(),
            })
        }
    };
}

/// Accept join request
#[post("/accept", format = "json", data = "<join_request>")]
pub async fn accept(db: &State<Db>, join_request: Json<JoinRequest>) -> Json<MemberShipResponse> {
    let join_request = join_request.into_inner();

    let maybe_vault = commons::find_vault(db, &join_request.member).await;

    return match maybe_vault {
        //user not found
        None => Json(MemberShipResponse {
            status: MembershipStatus::VaultNotFound,
            msg: "Vault not found".to_string(),
        }),
        Some(mut vault_doc) => {
            let candidate = join_request.candidate;
            if vault_doc.signatures.contains(&candidate) {
                remove_candidate_from_pending_queue(&candidate, &mut vault_doc);
                let vaults_col = db.vaults_col();
                update_vault(candidate.vault_name.clone(), vaults_col, vault_doc).await;
                return Json(MemberShipResponse {
                    status: MembershipStatus::AlreadyMember,
                    msg: "Device is already a member of the vault".to_string(),
                });
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

            Json(MemberShipResponse {
                status: MembershipStatus::Finished,
                msg: "Device has been added to members of the vault".to_string(),
            })
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
