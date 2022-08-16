mod Db;

use std::borrow::Borrow;
use mongodb::{Client, bson};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use crate::bson::Document;

use tracing_subscriber;
use tracing::{info, instrument};

#[macro_use]
extern crate rocket;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserRequest {
    user_name: String,
    public_key: String,
}

impl UserRequest {
    pub fn to_user_doc(&self) -> UserDoc {
        UserDoc { user_name: self.user_name.clone(), public_key: self.public_key.clone() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterResponse {
    result: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserDoc {
    user_name: String,
    public_key: String,
}

/// Request example:
/// curl -X POST http://localhost:8000/register -H 'Content-Type: application/json' -d '{"userName":"meta_dude","publicKey":"some_public_key"}'
///
#[instrument]
#[post("/register", format = "json", data = "<user_request_json>")]
async fn register(user_request_json: Json<UserRequest>) -> Json<RegisterResponse> {
    info!("Register user");

    let url = format!("mongodb://meta-secret-db:{}/", 27017);
    let client: Client = Client::with_uri_str(&url).await.unwrap();
    let db = client.database("meta-secret");
    let user_col = db.collection::<UserDoc>("users");

    let user_request = user_request_json.into_inner();

    //find user
    let maybe_user: Option<UserDoc> = user_col
        .find_one(bson::doc! { "userName": user_request.user_name.clone() }, None)
        .await.unwrap();

    match maybe_user {
        None => {
            //create a new user:
            user_col.insert_one(user_request.to_user_doc(), None)
                .await.unwrap();
            Json(RegisterResponse { result: String::from("user hase been created") })
        }
        Some(user_doc) => {
            //if user already exists
            //ask another device to allow a second device to be added to the cluster
            Json(RegisterResponse { result: String::from("error, user already exists") })
        }
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    let _rocket = rocket::build()
        .mount("/", routes![register])
        .launch()
        .await?;

    Ok(())
}
