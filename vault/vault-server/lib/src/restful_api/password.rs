use async_std::stream::StreamExt;
use mongodb::bson;
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::api::UserSignature;
use crate::api::api::{
    MessageStatus, MetaPasswordsResponse, MetaPasswordsStatus, PasswordRecoveryRequest,
};
use crate::db::MetaPasswordDoc;
use crate::db::{Db, SecretDistributionDoc};
use crate::restful_api::commons;

#[post("/recover", format = "json", data = "<recovery_request>")]
pub async fn claim_for_password_recovery(
    db: &State<Db>,
    recovery_request: Json<PasswordRecoveryRequest>,
) -> Json<MessageStatus> {
    let recovery_col = db.recovery_col();
    let record = recovery_request.into_inner();

    let result = recovery_col.insert_one(record, None).await;

    match result {
        Ok(_) => Json(MessageStatus::Ok),
        Err(_err) => Json(MessageStatus::Error {
            err: "Can't save data".to_string(),
        }),
    }
}

/*
#[post("/findClaimsForRecovery", format = "json", data = "<user_signature>")]
pub async fn find_claims_for_recovery(
    db: &State<Db>,
    user_signature: Json<UserSignature>,
) -> Json<Vec<SecretDistributionDoc>> {
    let recovery_col = db.recovery_col();

    let mut shares_docs = recovery_col
        .find(None, None)
        .await
        .unwrap();

    let mut shares = vec![];
    while let Some(share) = shares_docs.next().await {
        shares.push(share.unwrap());
    }

    Json(shares)
}
*/

#[post("/distribute", format = "json", data = "<distribution_request>")]
pub async fn distribute(
    db: &State<Db>,
    distribution_request: Json<SecretDistributionDoc>,
) -> Json<MessageStatus> {
    let distribution_request = distribution_request.into_inner();
    let meta_pass = distribution_request.meta_password.clone();

    let user_signature = meta_pass.user_sig.clone();
    let maybe_vault = commons::find_vault(db, &user_signature).await;

    match maybe_vault {
        None => Json(MessageStatus::Error {
            err: "Vault not found".to_string(),
        }),
        Some(_vault) => {
            //check that vault is correct

            let passwords_col = db.passwords_col();
            let meta_pass_filter = bson::doc! {
                "metaPassword.id.id": meta_pass.meta_password.id.id.clone(),
            };

            let meta_pass_db_record = passwords_col
                .find_one(meta_pass_filter, None)
                .await
                .unwrap();

            if meta_pass_db_record.is_none() {
                //create meta password record
                passwords_col.insert_one(meta_pass, None).await.unwrap();
            }

            let secrets_distribution_col = db.distribution_col();

            let result = secrets_distribution_col
                .insert_one(distribution_request.clone(), None)
                .await;

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
    db: &State<Db>,
    user_signature: Json<UserSignature>,
) -> Json<Vec<SecretDistributionDoc>> {
    let secrets_distribution_col = db.distribution_col();

    //find shares
    let secret_shares_filter = bson::doc! {
        "metaPassword.userSig.publicKey": user_signature.into_inner().public_key.base64_text
    };

    let mut shares_docs = secrets_distribution_col
        .find(secret_shares_filter, None)
        .await
        .unwrap();

    let mut shares = vec![];
    while let Some(share) = shares_docs.next().await {
        shares.push(share.unwrap());
    }

    //todo: delete shares from the database

    Json(shares)
}

#[post("/getMetaPasswords", format = "json", data = "<user_signature>")]
pub async fn passwords(
    db: &State<Db>,
    user_signature: Json<UserSignature>,
) -> Json<MetaPasswordsResponse> {
    let user_signature = user_signature.into_inner();
    let maybe_vault = commons::find_vault(db, &user_signature).await;

    let passwords_col = db.passwords_col();

    match maybe_vault {
        None => Json(MetaPasswordsResponse {
            status: MetaPasswordsStatus::VaultNotFound,
            passwords: vec![],
        }),
        Some(vault) => {
            let password_by_vault_filter = bson::doc! {
                "vault.vaultName": vault.vault_name.clone()
            };

            let mut meta_passwords_docs = passwords_col
                .find(password_by_vault_filter, None)
                .await
                .unwrap();

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
