use meta_secret_core::crypto::encoding::serialized_key_manager::SerializedKeyManager;
use meta_secret_core::crypto::keys::KeyManager;
use mongodb::bson;
use tracing::debug;

use crate::api::api::UserSignature;
use crate::crypto::crypto;
use crate::db::{Db, VaultDoc};

pub async fn find_vault(db: &Db, user_sig: &UserSignature) -> Option<VaultDoc> {
    let is_valid = crypto::verify(user_sig);
    if !is_valid {
        panic!("Can't pass signature verification");
    }

    let vault_filter = bson::doc! {
        "vaultName": user_sig.vault_name.clone()
    };

    let vaults_col = db.vaults_col();
    vaults_col.find_one(vault_filter, None).await.unwrap()
}

pub async fn get_server_key_manager(db: &Db) -> KeyManager {
    debug!("Get server key manager");

    let key_manager_col = db.key_manager_col();

    let keys_num = key_manager_col.count_documents(None, None).await.unwrap();
    match keys_num {
        0 => {
            debug!("Key manager not found. Generating a new one");
            let key_manager = KeyManager::generate();
            let serialized_km = SerializedKeyManager::from(&key_manager);
            key_manager_col
                .insert_one(serialized_km, None)
                .await
                .expect("Can't save key manager into mongodb");

            key_manager
        }
        1 => {
            let server_km_doc = key_manager_col.find_one(None, None).await.unwrap().unwrap();
            KeyManager::from(&server_km_doc)
        }
        _ => {
            panic!(
                "Invalid number of key managers: {}. Expected only one key manager",
                keys_num
            );
        }
    }
}

pub struct MetaState {
    pub db: Db,
    pub key_manager: KeyManager,
}
