use mongodb::{Client, Collection, Database};
use openssl::sha::Sha256;
use serde::{Deserialize, Serialize};

use crate::api::api::{
    EncryptedMessage, MetaPasswordRequest, PasswordRecoveryRequest, UserSignature,
};

#[derive(Clone, Debug)]
pub struct DbSchema {
    pub db_name: String,
    pub vault_col: String,
    pub secrets_distribution_col: String,
    pub secret_recovery_col: String,
    pub passwords_col: String,
}

impl Default for DbSchema {
    fn default() -> Self {
        DbSchema {
            db_name: "meta-secret".to_string(),
            vault_col: "vaults".to_string(),
            secrets_distribution_col: "secrets_distribution".to_string(),
            secret_recovery_col: "secret_recovery".to_string(),
            passwords_col: "passwords".to_string(),
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
    pub distribution_type: SecretDistributionType,
    pub meta_password: MetaPasswordRequest,
    pub secret_message: EncryptedMessage,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SecretDistributionType {
    Split,
    Recover,
}

#[derive(Clone, Debug)]
pub struct Db {
    pub db_schema: DbSchema,
    pub url: String,
    pub client: Client,
    pub db: Database,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MetaPasswordId {
    // SHA256 hash of salt
    pub id: String,
    // Random String up to 30 characters, must be unique
    pub salt: String,
    // human readable name given to the password
    pub name: String,
}

impl MetaPasswordId {
    pub fn new(name: String, salt: String) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(name.as_bytes());
        hasher.update("-".as_bytes());
        hasher.update(salt.as_bytes());

        let hash_bytes = hex::encode(hasher.finish());

        Self {
            id: hash_bytes,
            salt,
            name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetaPasswordDoc {
    pub id: MetaPasswordId,
    pub vault: VaultDoc,
}

impl Db {
    pub fn distribution_col(&self) -> Collection<SecretDistributionDoc> {
        let col_name = self.db_schema.secrets_distribution_col.as_str();
        self.db.collection::<SecretDistributionDoc>(col_name)
    }

    pub fn vaults_col(&self) -> Collection<VaultDoc> {
        let col_name = self.db_schema.vault_col.as_str();
        self.db.collection::<VaultDoc>(col_name)
    }

    pub fn passwords_col(&self) -> Collection<MetaPasswordRequest> {
        let col_name = self.db_schema.passwords_col.as_str();
        self.db.collection::<MetaPasswordRequest>(col_name)
    }

    pub fn recovery_col(&self) -> Collection<PasswordRecoveryRequest> {
        let col_name = self.db_schema.secret_recovery_col.as_str();
        self.db.collection::<PasswordRecoveryRequest>(col_name)
    }
}

/// https://github.com/testcontainers/testcontainers-rs/blob/dev/testcontainers/tests/images.rs
#[cfg(test)]
mod test {
    use mongodb::{bson, Client};
    use testcontainers::{clients, images::mongo};

    use crate::db::MetaPasswordId;

    #[test]
    fn meta_password_id() {
        let pass_id = MetaPasswordId::new("test".to_string(), "salt".to_string());
        assert_eq!(
            pass_id.id,
            "087280357dfdc5a3177e17b7424c7dfb1eab2d08ba3bedeb243dc51d5c18dc88".to_string()
        )
    }

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
