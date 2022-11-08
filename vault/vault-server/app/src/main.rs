use meta_secret_vault_server_lib::db::{Db, DbSchema};
use meta_secret_vault_server_lib::restful_api::commons::{get_server_key_manager, MetaState};
use meta_secret_vault_server_lib::restful_api::meta_secret_routes;
use mongodb::options::{ClientOptions, Credential};
use mongodb::Client;
use serde::Deserialize;
use std::fs;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let db_schema = DbSchema::default();

    let config_str = fs::read_to_string("mongodb-config.json").expect("Unable to read mongodb config file");
    let config: MongoDbConfig = serde_json::from_str(&config_str).unwrap();

    let mut opts = ClientOptions::parse(config.url.clone()).await.unwrap();
    opts.credential = Some(config.credential);

    let client: Client = Client::with_options(opts).unwrap();
    let mongo_db = client.database(db_schema.db_name.as_str());
    let db = Db {
        db_schema,
        url: config.url,
        client,
        db: mongo_db,
    };

    let key_manager = get_server_key_manager(&db).await;

    let meta_state = MetaState { db, key_manager };

    let _rocket = rocket::build()
        .manage(meta_state)
        .mount("/", meta_secret_routes())
        .launch()
        .await?;

    Ok(())
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct MongoDbConfig {
    credential: Credential,
    url: String,
}

#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::{routes, uri};

    use meta_secret_vault_server_lib::restful_api::basic::MAIN_MESSAGE;

    #[test]
    fn test_hi() {
        let rocket = rocket::build().mount("/", routes![meta_secret_vault_server_lib::restful_api::basic::hi]);

        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client
            .get(uri!(meta_secret_vault_server_lib::restful_api::basic::hi))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), String::from(MAIN_MESSAGE));
    }
}
