extern crate core;
#[macro_use]
extern crate rocket;

use mongodb::Client;
use rocket::futures::StreamExt;
use rocket::serde::json::Json;

use db::VaultDoc;

use crate::api::{
    EncryptedMessage, JoinRequest, RegistrationResponse, RegistrationStatus, UserSignature,
    VaultInfo, VaultInfoStatus,
};
use crate::crypto::digital_signature::DigitalSignatureRaw;
use crate::db::{Db, DbSchema, SecretDistributionDoc};

mod db;
mod crypto;
mod api;
mod restful_api;

#[get("/")]
pub async fn hi() -> String {
    "Hello Meta World!".to_string()
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
            hi,
            restful_api::register::register,
            restful_api::membership::accept,
            restful_api::membership::decline,
            restful_api::vault::get_vault,
            restful_api::password::distribute,
            restful_api::password::find_shares,
            restful_api::password::passwords,
            restful_api::password::add_meta_password
        ])
        .launch()
        .await?;

    Ok(())
}
