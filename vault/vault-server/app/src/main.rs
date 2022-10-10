extern crate core;
#[macro_use]
extern crate rocket;

use mongodb::Client;

use meta_secret_vault_server_lib::db::{Db, DbSchema};
use meta_secret_vault_server_lib::restful_api::meta_secret_routes;

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
        .mount("/", meta_secret_routes())
        .launch()
        .await?;

    Ok(())
}

#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    use meta_secret_vault_server_lib::restful_api::commons::MAIN_MESSAGE;

    #[test]
    fn test_hi() {
        let rocket = rocket::build().mount(
            "/",
            routes![meta_secret_vault_server_lib::restful_api::commons::hi],
        );

        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client
            .get(uri!(meta_secret_vault_server_lib::restful_api::commons::hi))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), String::from(MAIN_MESSAGE));
    }
}
