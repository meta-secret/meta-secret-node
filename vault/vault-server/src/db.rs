use mongodb::{Client, Collection, Database};
use serde::{Deserialize, Serialize};

use crate::api::EncryptedMessage;
use crate::UserSignature;

pub struct DbSchema {
    pub db_name: String,
    pub vault_col: String,
    pub secrets_distribution_col: String,
    pub passwords_col: String
}

impl Default for DbSchema {
    fn default() -> Self {
        DbSchema {
            db_name: "meta-secret".to_string(),
            vault_col: "vaults".to_string(),
            secrets_distribution_col: "secrets_distribution".to_string(),
            passwords_col: "passwords".to_string()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VaultDoc {
    pub vault_name: String,
    pub signatures: Vec<UserSignature>,
    pub pending_joins: Vec<UserSignature>,
    pub declined_joins: Vec<UserSignature>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SecretDistributionDoc {
    pub secret_message: EncryptedMessage,
}

pub struct Db {
    pub db_schema: DbSchema,
    pub url: String,
    pub client: Client,
    pub db: Database
}

/// Meta information about password
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetaPasswordDoc {
    pub id: String,
    pub vault: VaultDoc
}

impl Db {
    pub fn distribution_col(&self) -> Collection<SecretDistributionDoc> {
        let col_name = self.db_schema.secrets_distribution_col.as_str();
        self.db
            .collection::<SecretDistributionDoc>(col_name)
    }

    pub fn vaults_col(&self) -> Collection<VaultDoc> {
        let col_name = self.db_schema.vault_col.as_str();
        self.db.collection::<VaultDoc>(col_name)
    }

    pub fn passwords_col(&self) -> Collection<MetaPasswordDoc> {
        let col_name = self.db_schema.passwords_col.as_str();
        self.db.collection::<MetaPasswordDoc>(col_name)
    }
}

/// https://github.com/testcontainers/testcontainers-rs/blob/dev/testcontainers/tests/images.rs
#[cfg(test)]
mod test {
    use mongodb::{bson, Client};
    use testcontainers::{clients, images::mongo};

    #[tokio::test]
    async fn test_mongodb() {
        let _ = pretty_env_logger::try_init();
        let docker = clients::Cli::default();
        let node = docker.run(mongo::Mongo::default());
        let host_port = node.get_host_port_ipv4(27017);
        let url = format!("mongodb://localhost:{}/", host_port);

        let client: Client = Client::with_uri_str(&url).await.unwrap();
        let db = client.database("some_db");
        let coll = db.collection("some-coll");

        let doc = bson::doc! { "x": 42 };
        let insert_one_result = coll.insert_one(doc, None).await.unwrap();

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
