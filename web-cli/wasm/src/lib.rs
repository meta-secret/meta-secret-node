use meta_secret_core::crypto::encoding::serialized_key_manager::SerializedKeyManager;
use meta_secret_core::crypto::keys::KeyManager;
use meta_secret_core::recover_from_shares;
use meta_secret_core::sdk::vault::{UserInfo, UserSignature};
use meta_secret_core::shared_secret::data_block::common::SharedSecretConfig;
use meta_secret_core::shared_secret::shared_secret::{
    PlainText, SharedSecretEncryption, UserShareDto,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

mod utils;

/// Json utilities https://github.com/rustwasm/wasm-bindgen/blob/main/crates/js-sys/tests/wasm/JSON.rs

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
//pub fn generate_new_user(user_info_json: JsValue) -> JsValue {
pub fn generate_new_user(user_info_json: JsValue) -> JsValue {
    log("generate new user! wasm");

    let key_manager = KeyManager::generate();
    let serialized_km = SerializedKeyManager::from(&key_manager);
    let user_info: UserInfo = serde_wasm_bindgen::from_value(user_info_json).unwrap();
    let user_sig = UserSignature::get_from(&key_manager, &user_info);
    let js_user = JsUser {
        user_info,
        key_manager: serialized_km,
        user_signature: user_sig,
    };

    serde_wasm_bindgen::to_value(&js_user).unwrap()
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

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsUser {
    pub user_info: UserInfo,
    pub key_manager: SerializedKeyManager,
    user_signature: UserSignature,
}
