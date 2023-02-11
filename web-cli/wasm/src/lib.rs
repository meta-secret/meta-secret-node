use indexed_db_futures::prelude::*;
use meta_secret_core::crypto::keys::KeyManager;
use meta_secret_core::models::{
    DeviceInfo, FindSharesRequest, JoinRequest, MembershipRequestType, MetaVault,
    SecretDistributionType, UserCredentials, UserSecurityBox, UserSignature, VaultDoc,
};
use meta_secret_core::node::db::GenericRepo;
use meta_secret_core::node::server_api;
use meta_secret_core::recover_from_shares;
use meta_secret_core::sdk::api::MessageType;
use meta_secret_core::shared_secret::data_block::common::SharedSecretConfig;
use meta_secret_core::shared_secret::shared_secret::{
    PlainText, SharedSecretEncryption, UserShareDto,
};
use meta_secret_core::shared_secret::MetaDistributor;
use wasm_bindgen::prelude::*;
use web_sys::{IdbObjectStore as WebSysIdbObjectStore, IdbTransaction as WebSysIdbTransaction};

use crate::db::meta_vault::MetaVaultWasmRepo;
use crate::db::user_credentials::UserCredentialsWasmRepo;
use crate::db::{meta_vault, user_credentials};

mod db;
mod utils;

/// Json utilities https://github.com/rustwasm/wasm-bindgen/blob/main/crates/js-sys/tests/wasm/JSON.rs

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    pub async fn idbGet(db_name: &str, store_name: &str, key: &str) -> JsValue;
    pub async fn idbSave(db_name: &str, store_name: &str, key: &str, value: JsValue);
}

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

#[wasm_bindgen]
pub async fn get_vault() -> Result<JsValue, JsValue> {
    log("wasm: get vault!");

    let maybe_creds = internal::find_user_credentials()
        .await
        .map_err(JsError::from)?;

    match maybe_creds {
        Some(creds) => {
            let user_sig = creds.user_sig;
            let vault = server_api::get_vault(&user_sig)
                .await
                .map_err(JsError::from)?;

            let vault_js = serde_wasm_bindgen::to_value(&vault)?;
            Ok(vault_js)
        }
        None => Err(JsValue::from("Empty user credentials")),
    }
}

/*
///https://rustwasm.github.io/wasm-bindgen/examples/closures.html
#[wasm_bindgen]
pub async fn recover() -> Result<JsValue, JsValue> {
    //get security_box and user_sig from the database!!!!!
    log("wasm recover!");

    let db = open_db().await?;

    let tx: IdbTransaction =
        db.transaction_on_one_with_mode("security_box", IdbTransactionMode::Readwrite)?;

    tx.await.into_result()?;

    /*
    server_api::claim_for_password_recovery(&recovery_request)
    */

    Ok(JsValue::null())
}
 */

#[wasm_bindgen]
pub async fn sync(user_sig: JsValue) -> Result<JsValue, JsValue> {
    log("sync!");
    const STORE_NAME: &str = "secret_shares";

    let user_sig: UserSignature = serde_wasm_bindgen::from_value(user_sig)?;
    let request = FindSharesRequest {
        user_request_type: SecretDistributionType::Split,
        user_signature: Box::new(user_sig),
    };

    let shares_response = server_api::find_shares(&request)
        .await
        .map_err(JsError::from)?;

    let query_task = |_db: &IdbDatabase, tx: &WebSysIdbTransaction| {
        match shares_response.msg_type {
            MessageType::Ok => {
                log("wasm, sync: save shares to db");
                let shares_result = shares_response.data.unwrap();
                for share in shares_result.shares {
                    match share.distribution_type {
                        SecretDistributionType::Split => {
                            log("wasm, sync: split");

                            let store: WebSysIdbObjectStore = tx.object_store(STORE_NAME).unwrap();
                            let share_js = serde_wasm_bindgen::to_value(&share).unwrap();

                            // Add the employee to the store
                            log("save to db");
                            let pass_id = share.meta_password.meta_password.id.id;
                            let key = serde_wasm_bindgen::to_value(&pass_id).unwrap();
                            store.add_with_key(&share_js, &key).unwrap();
                        }
                        SecretDistributionType::Recover => {
                            //restore password
                            log("wasm, sync: recover");
                        }
                    }
                }
            }
            MessageType::Err => {
                let err_js = serde_wasm_bindgen::to_value(&shares_response.err.unwrap()).unwrap();
                log(format!("wasm, sync: error: {:?}", err_js).as_str());
                //Err(err_js);
            }
        }
    };

    log("wasm, sync: save to db");

    //save shares to db
    Ok(JsValue::null())
}

#[wasm_bindgen]
pub async fn cluster_distribution(
    pass_id: &str,
    pass: &str,
    security_box: JsValue,
    user_sig: JsValue,
    vault: JsValue,
) -> Result<JsValue, JsValue> {
    log("wasm: cluster distribution!!!!");

    let security_box: UserSecurityBox = serde_wasm_bindgen::from_value(security_box)?;
    let user_sig: UserSignature = serde_wasm_bindgen::from_value(user_sig)?;
    let vault: VaultDoc = serde_wasm_bindgen::from_value(vault)?;

    let distributor = MetaDistributor {
        security_box,
        user_sig,
        vault,
    };

    distributor
        .distribute(pass_id.to_string(), pass.to_string())
        .await;
    Ok(JsValue::from_str("ok"))
}

#[wasm_bindgen]
pub async fn membership(join_request: JsValue, request_type: JsValue) -> Result<JsValue, JsValue> {
    let join_request: JoinRequest = serde_wasm_bindgen::from_value(join_request)?;
    let request_type: MembershipRequestType = serde_wasm_bindgen::from_value(request_type)?;

    let log_msg = format!(
        "wasm: membership request. type: {:?}, request: {:?}",
        request_type, join_request
    );
    log(log_msg.as_str());

    let secrets = match request_type {
        MembershipRequestType::Accept => server_api::accept(&join_request).await.unwrap(),
        MembershipRequestType::Decline => server_api::decline(&join_request).await.unwrap(),
    };

    let secrets_js = serde_wasm_bindgen::to_value(&secrets)?;
    Ok(secrets_js)
}

#[wasm_bindgen]
pub async fn get_meta_passwords(user_sig: JsValue) -> Result<JsValue, JsValue> {
    log(format!("wasm: get meta passwords for: {:?}", user_sig).as_str());

    let user_sig = serde_wasm_bindgen::from_value(user_sig)?;
    log("wasm: get meta passwords");
    let secrets = server_api::get_meta_passwords(&user_sig)
        .await
        .map_err(JsError::from)?;

    let secrets_js = serde_wasm_bindgen::to_value(&secrets)?;
    Ok(secrets_js)
}

#[wasm_bindgen]
pub async fn register() -> Result<JsValue, JsValue> {
    let maybe_creds = internal::find_user_credentials()
        .await
        .map_err(JsError::from)?;

    match maybe_creds {
        Some(creds) => {
            let user_sig = creds.user_sig;
            let register_response = server_api::register(&user_sig)
                .await
                .map_err(JsError::from)?;

            let register_js = serde_wasm_bindgen::to_value(&register_response)?;
            Ok(register_js)
        }
        None => Err(JsValue::from("User credentials not found")),
    }
}

/// https://rustwasm.github.io/docs/wasm-bindgen/reference/arbitrary-data-with-serde.html
#[wasm_bindgen]
pub fn split(pass: &str) -> Result<JsValue, JsValue> {
    let plain_text = PlainText::from(pass);
    let config = SharedSecretConfig {
        number_of_shares: 3,
        threshold: 2,
    };
    let shared_secret = SharedSecretEncryption::new(config, &plain_text).map_err(JsError::from)?;

    let mut res: Vec<UserShareDto> = vec![];
    for share_index in 0..config.number_of_shares {
        let share: UserShareDto = shared_secret.get_share(share_index);
        res.push(share);
    }

    let shares_js = serde_wasm_bindgen::to_value(&res)?;
    Ok(shares_js)
}

#[wasm_bindgen]
pub fn restore_password(shares_json: JsValue) -> Result<JsValue, JsValue> {
    log("wasm: restore password, core functionality");

    let user_shares: Vec<UserShareDto> = serde_wasm_bindgen::from_value(shares_json)?;

    let plain_text = recover_from_shares(user_shares).map_err(JsError::from)?;
    Ok(JsValue::from_str(plain_text.text.as_str()))
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
