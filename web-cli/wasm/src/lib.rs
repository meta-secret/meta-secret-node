use js_sys::Promise;
use meta_secret_core::crypto::keys::KeyManager;
use meta_secret_core::models::{
    DeviceInfo, JoinRequest, MembershipRequestType, UserSecurityBox, UserSignature, VaultDoc,
};
use meta_secret_core::node::server_api;
use meta_secret_core::recover_from_shares;
use meta_secret_core::shared_secret::data_block::common::SharedSecretConfig;
use meta_secret_core::shared_secret::shared_secret::{
    PlainText, SharedSecretEncryption, UserShareDto,
};
use meta_secret_core::shared_secret::MetaDistributor;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
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
pub fn db_test() {
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
}

#[wasm_bindgen]
pub fn cluster_distribution(
    pass_id: &str,
    pass: &str,
    security_box: JsValue,
    user_sig: JsValue,
    vault: JsValue,
) -> Promise {
    log("wasm: cluster distribution!!!!");

    let security_box: UserSecurityBox = serde_wasm_bindgen::from_value(security_box).unwrap();
    let user_sig: UserSignature = serde_wasm_bindgen::from_value(user_sig).unwrap();
    let vault: VaultDoc = serde_wasm_bindgen::from_value(vault).unwrap();

    let distributor = MetaDistributor {
        security_box,
        user_sig,
        vault,
    };

    let task = internal::cluster_distribution(pass_id.to_string(), pass.to_string(), distributor);

    future_to_promise(task)
}

#[wasm_bindgen]
pub fn membership(join_request: JsValue, request_type: JsValue) -> Promise {
    let join_request: JoinRequest = serde_wasm_bindgen::from_value(join_request).unwrap();
    let request_type: MembershipRequestType = serde_wasm_bindgen::from_value(request_type).unwrap();

    let log_msg = format!(
        "wasm: membership request. type: {:?}, request: {:?}",
        request_type, join_request
    );
    log(log_msg.as_str());

    let task = internal::membership(join_request, request_type);
    future_to_promise(task)
}

#[wasm_bindgen]
pub fn get_meta_passwords(user_sig: JsValue) -> Promise {
    log(format!("wasm: get meta passwords for: {:?}", user_sig).as_str());

    let user_sig = serde_wasm_bindgen::from_value(user_sig).unwrap();
    let task = get_meta_passwords_from_server(user_sig);
    future_to_promise(task)
}

async fn get_meta_passwords_from_server(user_sig: UserSignature) -> Result<JsValue, JsValue> {
    log("wasm: get meta passwords");
    let secrets = server_api::get_meta_passwords(&user_sig).await.unwrap();
    Ok(serde_wasm_bindgen::to_value(&secrets).unwrap())
}

#[wasm_bindgen]
pub fn register(user_sig: JsValue) -> Promise {
    log(format!("wasm: register a new user! with: {:?}", user_sig).as_str());

    let user_sig = serde_wasm_bindgen::from_value(user_sig).unwrap();
    let task = server_registration(user_sig);
    future_to_promise(task)
}

async fn server_registration(user_sig: UserSignature) -> Result<JsValue, JsValue> {
    log("Registration on server!!!!");
    let register_async_task = server_api::register(&user_sig).await.unwrap();
    Ok(serde_wasm_bindgen::to_value(&register_async_task).unwrap())
}

#[wasm_bindgen]
pub fn get_vault(user_sig: JsValue) -> Promise {
    log("wasm: get vault!");

    let user_sig = serde_wasm_bindgen::from_value(user_sig).unwrap();
    let vault_future = get_vault_from_server(user_sig);
    future_to_promise(vault_future)
}

async fn get_vault_from_server(user_sig: UserSignature) -> Result<JsValue, JsValue> {
    let get_vault_task = server_api::get_vault(&user_sig).await.unwrap();
    Ok(serde_wasm_bindgen::to_value(&get_vault_task).unwrap())
}

#[wasm_bindgen]
pub fn generate_security_box(vault_name: &str) -> JsValue {
    log("wasm: generate new user");

    let security_box = KeyManager::generate_security_box(vault_name.to_string());
    serde_wasm_bindgen::to_value(&security_box).unwrap()
}

#[wasm_bindgen]
pub fn get_user_sig(security_box: JsValue, device: JsValue) -> JsValue {
    let security_box: UserSecurityBox = serde_wasm_bindgen::from_value(security_box).unwrap();
    let device: DeviceInfo = serde_wasm_bindgen::from_value(device).unwrap();

    let user_sig = security_box.get_user_sig(&device);
    serde_wasm_bindgen::to_value(&user_sig).unwrap()
}

/// https://rustwasm.github.io/docs/wasm-bindgen/reference/arbitrary-data-with-serde.html
#[wasm_bindgen]
pub fn split(pass: &str) -> JsValue {
    let plain_text = PlainText::from(pass);
    let config = SharedSecretConfig {
        number_of_shares: 3,
        threshold: 2,
    };
    let shared_secret = SharedSecretEncryption::new(config, &plain_text).unwrap();

    let mut res: Vec<UserShareDto> = vec![];
    for share_index in 0..config.number_of_shares {
        let share: UserShareDto = shared_secret.get_share(share_index);
        res.push(share);
    }

    serde_wasm_bindgen::to_value(&res).unwrap()
}

#[wasm_bindgen]
pub fn restore_password(shares_json: JsValue) -> String {
    log("wasm: restore password, core functionality");

    let user_shares: Vec<UserShareDto> = serde_wasm_bindgen::from_value(shares_json).unwrap();

    let maybe_plain_text = recover_from_shares(user_shares);

    match maybe_plain_text {
        Ok(plain_text) => plain_text.text,
        Err(_error) => {
            panic!("umerlo");
        }
    }
}

mod internal {
    use meta_secret_core::models::{JoinRequest, MembershipRequestType};
    use meta_secret_core::node::server_api;
    use meta_secret_core::shared_secret::MetaDistributor;
    use wasm_bindgen::JsValue;

    use crate::log;

    pub async fn membership(
        join_request: JoinRequest,
        request_type: MembershipRequestType,
    ) -> Result<JsValue, JsValue> {
        let secrets = match request_type {
            MembershipRequestType::Accept => server_api::accept(&join_request).await.unwrap(),
            MembershipRequestType::Decline => server_api::decline(&join_request).await.unwrap(),
        };

        Ok(serde_wasm_bindgen::to_value(&secrets).unwrap())
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
