use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use mongodb::{Client, options::ClientOptions};

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize)]
struct UserRequest {
    user_name: String,
    public_key: String
}

#[get("/register")]
async fn register() -> String {
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

#[cfg(test)]
mod test {
    use mongodb::{Client, bson, options::ClientOptions};
    use testcontainers::{clients, images::mongo};

    #[tokio::test]
    async fn test_mongodb() {
        let _ = pretty_env_logger::try_init();
        let docker = clients::Cli::default();
        let node = docker.run(mongo::Mongo::default());
        let host_port = node.get_host_port_ipv4(27017);
        let url = format!("mongodb://127.0.0.1:{}/", host_port);

        let client: Client = Client::with_uri_str(&url).await.unwrap();
        let db = client.database("some_db");
        let coll = db.collection("some-coll");

        let insert_one_result = coll.insert_one(bson::doc! { "x": 42 }, None).await.unwrap();
        assert!(!insert_one_result
            .inserted_id
            .as_object_id()
            .unwrap()
            .to_hex()
            .is_empty());

        let find_one_result: bson::Document = coll
            .find_one(bson::doc! { "x": 42 }, None)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(42, find_one_result.get_i32("x").unwrap())
    }
}
