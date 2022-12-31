use meta_secret_core::crypto::keys::KeyManager;
use meta_secret_core::models::{DeviceInfo, UserSecurityBox};
use meta_secret_core::recover_from_shares;
use meta_secret_core::shared_secret::data_block::common::SharedSecretConfig;
use meta_secret_core::shared_secret::shared_secret::{
    PlainText, SharedSecretEncryption, UserShareDto,
};
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
pub fn generate_security_box(vault_name: &str) -> JsValue {
    log("generate new user! wasm");

    let security_box = KeyManager::generate_security_box(vault_name.to_string());
    serde_wasm_bindgen::to_value(&security_box).unwrap()
}

#[wasm_bindgen]
pub fn get_user_sig(security_box: JsValue, device: JsValue) -> JsValue {
    log("generate new user! wasm");
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
