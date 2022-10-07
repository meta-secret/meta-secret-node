use async_std::stream::StreamExt;
use mongodb::bson;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::api::UserSignature;
use crate::api::api::{
    MessageStatus, MetaPasswordsRequest, MetaPasswordsResponse, MetaPasswordsStatus,
    PasswordRecoveryRequest,
};
use crate::db::{Db, SecretDistributionDoc};
use crate::db::{MetaPasswordDoc, MetaPasswordId};
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

#[post("/distribute", format = "json", data = "<encrypted_password_share>")]
pub async fn distribute(
    db: &State<Db>,
    encrypted_password_share: Json<SecretDistributionDoc>,
) -> Json<MessageStatus> {
    let secrets_distribution_col = db.distribution_col();

    let record = encrypted_password_share.into_inner();

    let result = secrets_distribution_col.insert_one(record, None).await;

    match result {
        Ok(_) => Json(MessageStatus::Ok),
        Err(_err) => Json(MessageStatus::Error {
            err: "Can't save data".to_string(),
        }),
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

            let id = MetaPasswordId {
                id: rand_id,
                name: "".to_string(),
            };
            let pass = MetaPasswordDoc { id, vault };

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
    use mongodb::Client;
    use rocket::http::{ContentType, Status};
    use rocket::local::asynchronous::{Client as RocketClient, LocalResponse};
    use rocket::{routes, uri};
    use testcontainers::images::mongo;
    use testcontainers::images::mongo::Mongo;
    use testcontainers::{clients, Container};

    use crate::api::api::{
        MetaPasswordsRequest, MetaPasswordsResponse, MetaPasswordsStatus, UserSignature,
    };
    use crate::db::{Db, DbSchema};

    #[rocket::async_test]
    async fn test_split() {
        let _ = pretty_env_logger::try_init();
        let docker = clients::Cli::default();
        let node: Container<Mongo> = docker.run(mongo::Mongo::default());

        let host_port = node.get_host_port_ipv4(27017);
        let url = format!("mongodb://localhost:{}/", host_port);

        let db_schema = DbSchema::default();

        let mongo_db_client: Client = Client::with_uri_str(&url).await.unwrap();
        let mongo_db = mongo_db_client.database(db_schema.db_name.as_str());

        let db = Db {
            db_schema,
            url,
            client: mongo_db_client,
            db: mongo_db,
        };

        let rocket = rocket::build()
            .manage(db.clone())
            .mount("/", routes![super::add_meta_password, super::passwords,]);

        let rocket_client = RocketClient::tracked(rocket)
            .await
            .expect("valid rocket instance");

        let pass = MetaPasswordsRequest {
            user_sig: UserSignature::generate_default_for_tests(),
            name: "test".to_string(),
        };

        let add_meta_password_response = rocket_client
            .post(uri!(super::add_meta_password))
            .header(ContentType::JSON)
            .body(serde_json::to_string_pretty(&pass).unwrap())
            .dispatch()
            .await;

        assert_eq!(add_meta_password_response.status(), Status::Ok);

        let passwords_col = &db.passwords_col();
        let count = passwords_col.find(None, None).await.into_iter().count();
        assert_eq!(count, 1);

        //let meta_pass_doc= &db.passwords_col().find(None, None)
        //    .await
        //    .unwrap();

        let passwords_response: LocalResponse = rocket_client
            .post(uri!(super::passwords))
            .header(ContentType::JSON)
            .body(serde_json::to_string_pretty(&pass.user_sig).unwrap())
            .dispatch()
            .await;

        let response = passwords_response
            .into_json::<MetaPasswordsResponse>()
            .await
            .unwrap();

        assert_eq!(response.status, MetaPasswordsStatus::VaultNotFound);
    }
}
