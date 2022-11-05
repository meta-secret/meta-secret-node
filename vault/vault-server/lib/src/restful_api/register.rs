use mongodb::bson;
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::api::UserSignature;
use crate::api::api::{RegistrationResponse, RegistrationStatus};
use crate::restful_api::commons;
use crate::restful_api::commons::MetaState;

/// Register a new distributed vault
#[post("/register", format = "json", data = "<register_request>")]
pub async fn register(state: &State<MetaState>, register_request: Json<UserSignature>) -> Json<RegistrationResponse> {
    let user_sig = register_request.into_inner();
    let maybe_vault = commons::find_vault(&state.db, &user_sig).await;

    match maybe_vault {
        None => {
            //create a new user:
            let vaults_col = state.db.vaults_col();
            vaults_col
                .insert_one(user_sig.to_initial_vault_doc(), None)
                .await
                .unwrap();

            Json(RegistrationResponse {
                status: RegistrationStatus::Registered,
                result: "Vault has been created".to_string(),
            })
        }
        Some(mut vault_doc) => {
            //if vault already exists
            if vault_doc.signatures.contains(&user_sig) {
                return Json(RegistrationResponse {
                    status: RegistrationStatus::Registered,
                    result: "Vault already exists and you are one of the members".to_string(),
                });
            }

            vault_doc.pending_joins.push(user_sig.clone());

            let vault_name = user_sig.vault_name.clone();
            let filter = bson::doc! {
                "vaultName": vault_name
            };
            let vaults_col = state.db.vaults_col();
            vaults_col.replace_one(filter, vault_doc.clone(), None).await.unwrap();

            Json(RegistrationResponse {
                status: RegistrationStatus::AlreadyExists,
                result: "Added to pending requests".to_string(),
            })
        }
    }
}
