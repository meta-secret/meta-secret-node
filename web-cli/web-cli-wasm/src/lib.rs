mod utils;

use meta_secret_core::shared_secret::data_block::common::SharedSecretConfig;
use meta_secret_core::shared_secret::shared_secret::{PlainText, SharedSecretEncryption, UserShareDto};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

/// https://rustwasm.github.io/docs/wasm-bindgen/reference/arbitrary-data-with-serde.html
#[wasm_bindgen]
pub fn split(pass: &str) -> JsValue {
    let plain_text = PlainText::from_str(String::from(pass));
    let config = SharedSecretConfig {
        number_of_shares: 3,
        threshold: 2
    };
    let shared_secret = SharedSecretEncryption::new(config, &plain_text);

    let mut res: Vec<UserShareDto> = vec![];
    for share_index in 0..config.number_of_shares {
        let share: UserShareDto = shared_secret.get_share(share_index);
        let share_json = serde_json::to_string_pretty(&share)
            .unwrap();

        res.push(share);
    }

    return JsValue::from_serde(&res).unwrap();
}
