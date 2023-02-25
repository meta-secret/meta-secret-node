use meta_secret_core::crypto::keys::KeyManager;
use meta_secret_core::models::{DeviceInfo, MetaVault, UserCredentials};
use meta_secret_core::node::db::GenericRepo;
use wasm_bindgen::prelude::*;

use crate::db::meta_vault::MetaVaultWasmRepo;
use crate::db::user_credentials::UserCredentialsWasmRepo;
use crate::db::{meta_vault, user_credentials};
use crate::log;

#[wasm_bindgen]
pub async fn get_meta_vault() -> Result<Option<JsValue>, JsValue> {
    let maybe_meta_vault = internal::find_meta_vault().await.map_err(JsError::from)?;

    if let Some(meta_vault) = maybe_meta_vault {
        let meta_vault_js = serde_wasm_bindgen::to_value(&meta_vault)?;
        Ok(Some(meta_vault_js))
    } else {
        Ok(None)
    }
}

#[wasm_bindgen]
pub async fn create_meta_vault(vault_name: &str, device_name: &str) -> Result<JsValue, JsValue> {
    let meta_vault_repo = MetaVaultWasmRepo {};

    let device = DeviceInfo {
        device_id: meta_secret_core::crypto::utils::generate_hash(),
        device_name: device_name.to_string(),
    };

    let meta_vault = MetaVault {
        vault_name: vault_name.to_string(),
        device: Box::new(device),
    };

    meta_vault_repo
        .save(meta_vault::store_conf::KEY_NAME, &meta_vault)
        .await
        .map_err(JsError::from)?;

    let meta_vault_js = serde_wasm_bindgen::to_value(&meta_vault)?;

    Ok(meta_vault_js)
}

#[wasm_bindgen]
pub async fn generate_user_credentials() -> Result<(), JsValue> {
    log("wasm: generate a new security box");

    let maybe_meta_vault = internal::find_meta_vault().await.map_err(JsError::from)?;

    match maybe_meta_vault {
        Some(meta_vault) => {
            let security_box = KeyManager::generate_security_box(meta_vault.vault_name);
            let user_sig = security_box.get_user_sig(&meta_vault.device);
            let creds = UserCredentials::new(security_box, user_sig);

            let creds_repo = UserCredentialsWasmRepo {};
            creds_repo
                .save(user_credentials::store_conf::KEY_NAME, &creds)
                .await
                .map_err(JsError::from)?;

            Ok(())
        }
        None => {
            let err_msg =
                JsValue::from("The parameters have not yet set for the vault. Empty meta vault");
            Err(err_msg)
        }
    }
}

pub mod internal {
    use meta_secret_core::models::{MetaVault, UserCredentials};

    use crate::db::meta_vault::MetaVaultWasmRepo;
    use crate::db::user_credentials::UserCredentialsWasmRepo;
    use crate::db::WasmDbError;

    pub async fn find_meta_vault() -> Result<Option<MetaVault>, WasmDbError> {
        let meta_vault_repo = MetaVaultWasmRepo {};
        meta_vault_repo.find_meta_vault().await
    }

    pub async fn find_user_credentials() -> Result<Option<UserCredentials>, WasmDbError> {
        let repo = UserCredentialsWasmRepo {};
        repo.find_user_credentials().await
    }
}
