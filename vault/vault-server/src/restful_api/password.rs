use mongodb::bson;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rocket::serde::json::Json;
use rocket::State;

use crate::{Db, EncryptedMessage, SecretDistributionDoc, StreamExt, UserSignature};
use crate::api::{MetaPasswordsRequest, MetaPasswordsResponse, MetaPasswordsStatus, SecretDistributionStatus};
use crate::db::{MetaPasswordDoc, MetaPasswordId};
use crate::restful_api::commons;

#[post("/distribute", format = "json", data = "<encrypted_password_share>")]
pub async fn distribute(
    db: &State<Db>,
    encrypted_password_share: Json<SecretDistributionDoc>,
) -> Json<SecretDistributionStatus> {
    let secrets_distribution_col = db.distribution_col();

    //create a new user:
    let record = encrypted_password_share.into_inner();

    let result = secrets_distribution_col
        .insert_one(record, None)
        .await;

    match result {
        Ok(_) => {
            Json(SecretDistributionStatus::Ok)
        }
        Err(err) => {
            Json(SecretDistributionStatus::Error { err: "Can't save data".to_string() })
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
                meta_passwords.push(meta_password.unwrap());
            }

            Json(MetaPasswordsResponse {
                status: MetaPasswordsStatus::Ok,
                passwords: meta_passwords,
            })
        }
    }
}

#[post("/addMetaPassword", format = "json", data = "<meta_password_request>")]
pub async fn add_meta_password(
    db: &State<Db>,
    meta_password_request: Json<MetaPasswordsRequest>,
) -> Json<MetaPasswordsResponse> {
    let user_signature = meta_password_request.into_inner().user_sig;
    let maybe_vault = commons::find_vault(db, &user_signature).await;

    match maybe_vault {
        None => Json(MetaPasswordsResponse {
            status: MetaPasswordsStatus::VaultNotFound,
            passwords: vec![],
        }),
        Some(vault) => {
            let rand_id: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .map(char::from)
                .collect();

            let pass = MetaPasswordDoc { id: MetaPasswordId {id: rand_id, name: "".to_string() }, vault };

            let passwords_col = db.passwords_col();
            passwords_col.insert_one(pass.clone(), None).await.unwrap();

            Json(MetaPasswordsResponse {
                status: MetaPasswordsStatus::Ok,
                passwords: vec![pass],
            })
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn yay() {}
}
