#[macro_use]
extern crate rocket;

use std::borrow::Borrow;

use ed25519_dalek::{Keypair, PublicKey, Signature, Verifier};
use mongodb::{bson, Client};
use rand::rngs::OsRng;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use tracing::info;
use tracing_subscriber;
use tracing_subscriber::FmtSubscriber;

use api::{RegisterResponse, UserDoc, UserRequest};

use crate::bson::Document;

mod db;
mod crypto;
mod api;


/// Request example:
/// curl -X POST http://localhost:8000/register -H 'Content-Type: application/json' -d '{"userName":"test_user","publicKey":"922JB+F8ktWuQxeHWzlHZ3XH3/5/2EGma0aHa4Yu1FU=","signatureOfUserName":"c92vK/pMACBEZKV76DSirQuw38PcDcOYjBrotVM00x35AhwWrW4POLhdh3+Ssaw0Wg8pUL1EWSY6+2WjbCNiDA=="}'
///
#[post("/register", format = "json", data = "<user_request>")]
async fn register(user_request: Json<UserRequest>) -> Json<RegisterResponse> {
    info!("Register user");

    let url = format!("mongodb://meta-secret-db:{}/", 27017);
    let client: Client = Client::with_uri_str(&url).await.unwrap();
    let db = client.database("meta-secret");
    let user_col = db.collection::<UserDoc>("users");

    let user_request = user_request.into_inner();

    println!("verify: {:?}", user_request);
    let is_valid = crypto::verify(&user_request);

    if !is_valid {
        panic!("Can't pass signature verification");
    }

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
    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|_err| eprintln!("Unable to set global default subscriber"));
    //.expect("TODO: can't configure logger");

    let _rocket = rocket::build()
        .mount("/", routes![register])
        .launch()
        .await?;

    Ok(())
}
