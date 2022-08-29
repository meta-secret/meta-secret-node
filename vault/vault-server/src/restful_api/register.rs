use mongodb::bson;
use rocket::serde::json::Json;
use rocket::State;

use crate::{Db, RegistrationResponse, RegistrationStatus, UserSignature, VaultDoc};
use crate::crypto::crypto;

/// Register a new distributed vault
#[post("/register", format = "json", data = "<register_request>")]
pub async fn register(register_request: Json<UserSignature>, db: &State<Db>) -> Json<RegistrationResponse> {
    info!("Register a new vault or join");
    let user_sig = register_request.into_inner();
    info!("verify: {:?}", user_sig);

    let is_valid = crypto::verify(&user_sig);
    if !is_valid {
        panic!("Can't pass signature verification");
    }

    //find user
    let vault_filter = bson::doc! {
        "vaultName": user_sig.vault_name.clone()
    };

    let vaults_col = db.vaults_col();
    let maybe_vault: Option<VaultDoc> = vaults_col
        .find_one(vault_filter, None)
        .await
        .unwrap();

    return match maybe_vault {
        None => {
            //create a new user:
            vaults_col.insert_one(user_sig.to_initial_vault_doc(), None)
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