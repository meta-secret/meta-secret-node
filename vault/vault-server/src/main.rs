use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize)]
struct UserRequest {
    user_name: String,
    public_key: String
}

#[get("/register")]
async fn register() -> String {
    // check if user is already present in the database
    //
    //database?
    String::from("yay")
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![register])
        .launch()
        .await?;

    Ok(())
}
