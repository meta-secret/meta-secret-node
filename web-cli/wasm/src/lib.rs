use js_sys::Promise;
use meta_secret_core::crypto::keys::KeyManager;
use meta_secret_core::models::{
    DeviceInfo, JoinRequest, MembershipRequestType, SecretDistributionType, UserSecurityBox,
    UserSignature, VaultDoc,
};
use meta_secret_core::node::server_api;
use meta_secret_core::recover_from_shares;
use meta_secret_core::sdk::api::MessageType;
use meta_secret_core::shared_secret::data_block::common::SharedSecretConfig;
use meta_secret_core::shared_secret::shared_secret::{
    PlainText, SharedSecretEncryption, UserShareDto,
};
use meta_secret_core::shared_secret::MetaDistributor;
use wasm_bindgen::prelude::*;
use web_sys::{IdbDatabase, IdbTransaction};

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
}

#[wasm_bindgen]
pub async fn sync(user_sig: JsValue) -> Result<JsValue, JsValue> {
    let user_sig: UserSignature = serde_wasm_bindgen::from_value(user_sig)?;
    let shares_response = server_api::find_shares(&user_sig)
        .await
        .map_err(JsError::from)?;

    match shares_response.msg_type {
        MessageType::Ok => {
            let shares = shares_response.data.unwrap();
            for share in shares.shares {
                match share.distribution_type {
                    SecretDistributionType::Split => {
                        //save to db
                    }
                    SecretDistributionType::Recover => {
                        //restore password
                    }
                }
            }
        }
        MessageType::Err => {
            let err_js = serde_wasm_bindgen::to_value(&shares_response.err.unwrap())?;
            //Err(err_js);
        }
    }

    //save shares to db
    Ok(JsValue::null())
}

#[wasm_bindgen]
pub fn db_test() -> Result<JsValue, JsValue> {
    //https://rustwasm.github.io/wasm-bindgen/examples/closures.html

    const STORE_NAME: &str = "meta_passwords";

    let query_task = Box::from(|_db: &IdbDatabase, tx: &IdbTransaction| {
        let store = tx.object_store(STORE_NAME).unwrap();

        let user = serde_json::json!({
            "name": "meta_user",
            "email": "fake@meta-secret.org",
        });
        // Convert it to `JsValue`
        let user = serde_wasm_bindgen::to_value(&user).unwrap();

        // Add the employee to the store
        log("save to db");
        let key = serde_wasm_bindgen::to_value("meta_user").unwrap();
        store.add_with_key(&user, &key).unwrap();
    });

    db::tx(&[STORE_NAME], query_task);
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

    internal::cluster_distribution(pass_id.to_string(), pass.to_string(), distributor).await
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

    internal::membership(join_request, request_type).await
}

#[wasm_bindgen]
pub async fn get_meta_passwords(user_sig: JsValue) -> Result<JsValue, JsValue> {
    log(format!("wasm: get meta passwords for: {:?}", user_sig).as_str());

    let user_sig = serde_wasm_bindgen::from_value(user_sig)?;
    get_meta_passwords_from_server(user_sig).await
}

async fn get_meta_passwords_from_server(user_sig: UserSignature) -> Result<JsValue, JsValue> {
    log("wasm: get meta passwords");
    let secrets = server_api::get_meta_passwords(&user_sig)
        .await
        .map_err(JsError::from)?;

    let secrets_js = serde_wasm_bindgen::to_value(&secrets)?;
    Ok(secrets_js)
}

#[wasm_bindgen]
pub async fn register(user_sig: JsValue) -> Result<JsValue, JsValue> {
    log(format!("wasm: register a new user! with: {:?}", user_sig).as_str());

    let user_sig = serde_wasm_bindgen::from_value(user_sig)?;
    server_registration(user_sig).await
}

async fn server_registration(user_sig: UserSignature) -> Result<JsValue, JsValue> {
    log("Registration on server!!!!");
    let register_response = server_api::register(&user_sig)
        .await
        .map_err(JsError::from)?;

    let register_js = serde_wasm_bindgen::to_value(&register_response)?;
    Ok(register_js)
}

#[wasm_bindgen]
pub async fn get_vault(user_sig: JsValue) -> Result<JsValue, JsValue> {
    log("wasm: get vault!");

    let user_sig = serde_wasm_bindgen::from_value(user_sig)?;
    get_vault_from_server(user_sig).await
}

async fn get_vault_from_server(user_sig: UserSignature) -> Result<JsValue, JsValue> {
    let vault = server_api::get_vault(&user_sig)
        .await
        .map_err(JsError::from)?;
    let vault_js = serde_wasm_bindgen::to_value(&vault)?;
    Ok(vault_js)
}

#[wasm_bindgen]
pub fn generate_security_box(vault_name: &str) -> Result<JsValue, JsValue> {
    log("wasm: generate new user");

    let security_box = KeyManager::generate_security_box(vault_name.to_string());
    let security_box_js = serde_wasm_bindgen::to_value(&security_box)?;
    Ok(security_box_js)
}

#[wasm_bindgen]
pub fn get_user_sig(security_box: JsValue, device: JsValue) -> Result<JsValue, JsValue> {
    let security_box: UserSecurityBox = serde_wasm_bindgen::from_value(security_box)?;
    let device: DeviceInfo = serde_wasm_bindgen::from_value(device)?;

    let user_sig = security_box.get_user_sig(&device);
    let user_sig_js = serde_wasm_bindgen::to_value(&user_sig)?;
    Ok(user_sig_js)
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

mod internal {
    use meta_secret_core::models::{JoinRequest, MembershipRequestType};
    use meta_secret_core::node::server_api;
    use meta_secret_core::shared_secret::MetaDistributor;
    use wasm_bindgen::JsValue;

    pub async fn membership(
        join_request: JoinRequest,
        request_type: MembershipRequestType,
    ) -> Result<JsValue, JsValue> {
        let secrets = match request_type {
            MembershipRequestType::Accept => server_api::accept(&join_request).await.unwrap(),
            MembershipRequestType::Decline => server_api::decline(&join_request).await.unwrap(),
        };

        let secrets_js = serde_wasm_bindgen::to_value(&secrets)?;
        Ok(secrets_js)
    }

    pub async fn cluster_distribution(
        pass_id: String,
        pass: String,
        distributor: MetaDistributor,
    ) -> Result<JsValue, JsValue> {
        distributor
            .distribute(pass_id.to_string(), pass.to_string())
            .await;
        Ok(JsValue::from_str("ok"))
    }
}
