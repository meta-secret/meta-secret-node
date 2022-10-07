extern crate core;
#[macro_use]
extern crate rocket;

use mongodb::Client;
use rocket::futures::StreamExt;

use meta_secret_vault_server_lib::db::{Db, DbSchema};
use meta_secret_vault_server_lib::restful_api;

const MAIN_MESSAGE: &'static str = "Hello Meta World!";

#[get("/")]
pub async fn hi() -> String {
    String::from(MAIN_MESSAGE)
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
        db: mongo_db,
    };

    let _rocket = rocket::build()
        .manage(db)
        .mount(
            "/",
            routes![
                hi,
                restful_api::register::register,
                restful_api::membership::accept,
                restful_api::membership::decline,
                restful_api::vault::get_vault,
                restful_api::password::distribute,
                restful_api::password::find_shares,
                restful_api::password::passwords,
                restful_api::password::add_meta_password
            ],
        )
        .launch()
        .await?;

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::MAIN_MESSAGE;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn hi() {
        let rocket = rocket::build().mount("/", routes![super::hi]);

        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get(uri!(super::hi)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), String::from(MAIN_MESSAGE));
    }
}
