use async_std::stream::StreamExt;
use meta_secret_core::crypto::key_pair::KeyPair;
use mongodb::bson;

use rocket::post;
use rocket::serde::json::Json;
use rocket::State;
use tracing::{debug, error};

use crate::api::api::UserSignature;
use crate::api::api::{MessageStatus, MetaPasswordsResponse, MetaPasswordsStatus, PasswordRecoveryRequest};
use crate::db::MetaPasswordDoc;
use crate::db::SecretDistributionDoc;
use crate::restful_api::commons;
use crate::restful_api::commons::MetaState;

#[post("/claimForPasswordRecovery", format = "json", data = "<recovery_request>")]
pub async fn claim_for_password_recovery(
    state: &State<MetaState>,
    recovery_request: Json<PasswordRecoveryRequest>,
) -> Json<MessageStatus> {
    let recovery_col = state.db.recovery_col();
    let recovery_request = recovery_request.into_inner();

    //check if a recovery request is for the cloud account
    let provider_pk = recovery_request.provider.public_key.clone();

    if state.key_manager.dsa.public_key() == provider_pk {
        let cloud_storage_col = state.db.cloud_storage_col();
        let filter = bson::doc! {
            "metaPassword.metaPassword.id.id": recovery_request.id.id.clone(),
        };
        let share = cloud_storage_col.find_one(filter, None).await.unwrap().unwrap();

        distribute(state, Json(share)).await
    } else {
        let result = recovery_col.insert_one(recovery_request, None).await;
        match result {
            Ok(_) => Json(MessageStatus::Ok),
            Err(_err) => Json(MessageStatus::Error {
                err: "Can't save data".to_string(),
            }),
        }
    }
}

#[post("/findPasswordRecoveryClaims", format = "json", data = "<user_signature>")]
pub async fn find_password_recovery_claims(
    state: &State<MetaState>,
    user_signature: Json<UserSignature>,
) -> Json<Vec<PasswordRecoveryRequest>> {
    let recovery_col = state.db.recovery_col();
    let user_signature = user_signature.into_inner();

    //find shares
    let filter = bson::doc! {
        "provider.publicKey.base64Text": user_signature.public_key.base64_text.clone()
    };

    let mut claims_col = recovery_col.find(filter, None).await.unwrap();

    let mut claims = vec![];
    while let Some(claim) = claims_col.next().await {
        let claim = claim.unwrap();
        claims.push(claim);
    }

    let filter = bson::doc! {
        "provider.publicKey.base64Text": user_signature.public_key.base64_text
    };
    recovery_col.delete_many(filter, None).await.unwrap();

    Json(claims)
}

#[post("/distribute", format = "json", data = "<distribution_request>")]
pub async fn distribute(
    state: &State<MetaState>,
    distribution_request: Json<SecretDistributionDoc>,
) -> Json<MessageStatus> {
    let distribution_request = distribution_request.into_inner();
    let auth_data = &distribution_request.secret_message.encrypted_text.auth_data;
    debug!(
        "Distribute password share: {:?}, channel: {:?}",
        &distribution_request.meta_password.meta_password.id, auth_data.channel
    );

    let meta_pass = distribution_request.meta_password.clone();

    let user_signature = meta_pass.user_sig.clone();
    let maybe_vault = commons::find_vault(&state.db, &user_signature).await;

    match maybe_vault {
        None => {
            error!(
                "Password distribution error. Vault not found: {:?}",
                &user_signature.vault_name
            );
            Json(MessageStatus::Error {
                err: "Vault not found".to_string(),
            })
        }
        Some(_vault) => {
            //TODO: check that the vault is correct

            let passwords_col = state.db.meta_passwords_col();
            let meta_pass_id = &meta_pass.meta_password.id;
            let filter = bson::doc! {
                "metaPassword.id.id": meta_pass_id.id.clone(),
            };

            let meta_pass_db_record = passwords_col.find_one(filter, None).await.unwrap();

            if meta_pass_db_record.is_none() {
                debug!("Create a new meta password record: {:?}", &meta_pass_id);
                passwords_col.insert_one(&meta_pass, None).await.unwrap();
            }

            //Check if the password share is a cloud one
            let receiver_pk = distribution_request
                .secret_message
                .encrypted_text
                .auth_data
                .channel
                .receiver
                .clone();

            let result = if state.key_manager.transport_key_pair.public_key() == receiver_pk {
                debug!("Save a password share to the cloud");
                let cloud_storage_col = state.db.cloud_storage_col();
                cloud_storage_col.insert_one(distribution_request.clone(), None).await
            } else {
                debug!("Save a password share to the distribution collection");
                let secrets_distribution_col = state.db.distribution_col();

                secrets_distribution_col
                    .insert_one(distribution_request.clone(), None)
                    .await
            };

            match result {
                Ok(_) => Json(MessageStatus::Ok),
                Err(_err) => Json(MessageStatus::Error {
                    err: "Can't save password share".to_string(),
                }),
            }
        }
    }
}

#[post("/findShares", format = "json", data = "<user_signature>")]
pub async fn find_shares(
    state: &State<MetaState>,
    user_signature: Json<UserSignature>,
) -> Json<Vec<SecretDistributionDoc>> {
    let user_signature = user_signature.into_inner();
    let dsa_key = user_signature.public_key.base64_text;

    debug!("Find shares for device: {:?}", &dsa_key);

    let secrets_distribution_col = state.db.distribution_col();

    //find shares
    let filter = bson::doc! {
        "secretMessage.receiver.publicKey.base64Text": dsa_key
    };

    let mut shares_docs = secrets_distribution_col.find(filter.clone(), None).await.unwrap();

    let mut shares = vec![];
    while let Some(share) = shares_docs.next().await {
        let share = share.unwrap();
        shares.push(share);
    }

    secrets_distribution_col.delete_many(filter, None).await.unwrap();

    Json(shares)
}

#[post("/getMetaPasswords", format = "json", data = "<user_signature>")]
pub async fn passwords(state: &State<MetaState>, user_signature: Json<UserSignature>) -> Json<MetaPasswordsResponse> {
    let user_signature = user_signature.into_inner();
    let maybe_vault = commons::find_vault(&state.db, &user_signature).await;

    let passwords_col = state.db.meta_passwords_col();

    match maybe_vault {
        None => Json(MetaPasswordsResponse {
            status: MetaPasswordsStatus::VaultNotFound,
            passwords: vec![],
        }),
        Some(vault) => {
            let password_by_vault_filter = bson::doc! {
                "vault.vaultName": vault.vault_name.clone()
            };

            let mut meta_passwords_docs = passwords_col.find(password_by_vault_filter, None).await.unwrap();

            let mut meta_passwords: Vec<MetaPasswordDoc> = vec![];
            while let Some(meta_password) = meta_passwords_docs.next().await {
                meta_passwords.push(meta_password.unwrap().meta_password);
            }

            Json(MetaPasswordsResponse {
                status: MetaPasswordsStatus::Ok,
                passwords: meta_passwords,
            })
        }
    }
}
