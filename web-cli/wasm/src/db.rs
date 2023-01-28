use std::fmt::Error;

use js_sys::Array;
use meta_secret_core::models::{UserSecurityBox, UserSignature};
use meta_secret_core::node::db::BasicRepo;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{
    Event, IdbDatabase, IdbFactory, IdbOpenDbRequest, IdbRequest, IdbTransaction, Window,
};

use crate::log;

pub fn tx<T: AsRef<str>>(store_names: &[T], task: Box<dyn FnOnce(&IdbDatabase, &IdbTransaction)>) {
    let factory: IdbFactory = web_sys::window().unwrap().indexed_db().unwrap().unwrap();
    let open_db_request: IdbOpenDbRequest = factory.open("meta_secret").unwrap();

    let store_names_2: Array = store_names
        .iter()
        .map(|s| JsValue::from(s.as_ref()))
        .collect();

    let on_upgrade_action: Closure<dyn FnMut(Event)> = Closure::once(move |event: Event| {
        log("db onUpgrade event");

        let target = event.target().unwrap();
        let request: &IdbRequest = AsRef::<JsValue>::as_ref(&target).unchecked_ref();
        let db: IdbDatabase = request.result().unwrap().unchecked_into();

        for store_name in store_names_2.iter() {
            db.create_object_store(store_name.as_string().unwrap().as_str())
                .unwrap();
        }
    });
    open_db_request.set_onupgradeneeded(Some(on_upgrade_action.as_ref().unchecked_ref()));

    let store_names: Array = store_names
        .iter()
        .map(|s| JsValue::from(s.as_ref()))
        .collect();

    let on_success_action: Closure<dyn FnMut(Event)> = Closure::once(move |event: Event| {
        let target = event.target().unwrap();
        let request: &IdbRequest = AsRef::<JsValue>::as_ref(&target).unchecked_ref();

        let raw_db = request.result().unwrap();
        let db: &IdbDatabase = raw_db.as_ref().unchecked_ref();

        let rw_mode = web_sys::IdbTransactionMode::Readwrite;
        let tx: IdbTransaction = db
            .transaction_with_str_sequence_and_mode(&store_names, rw_mode)
            .unwrap();

        task(db, &tx);

        tx.commit().unwrap();
    });

    open_db_request.set_onsuccess(Some(on_success_action.as_ref().unchecked_ref()));

    //todo fix memory leaks (see wasm_bindgen::Closure doc and https://rustwasm.github.io/wasm-bindgen/reference/passing-rust-closures-to-js.html)
    on_success_action.forget();
    on_upgrade_action.forget();
}

fn yaya() {
    let dbb = SecurityBoxRepo {};
    let sec_box = UserSecurityBox {
        vault_name: "".to_string(),
        signature: Box::new(Default::default()),
        key_manager: Box::new(Default::default()),
    };
    let boxx = dbb.save(&sec_box);
    let sec_boxx: Result<UserSecurityBox, Error> = dbb.get();
    let sig_sig: Result<UserSignature, Error> = dbb.get();

    let sig = UserSignature {
        vault_name: "".to_string(),
        device: Box::new(Default::default()),
        public_key: Box::new(Default::default()),
        transport_public_key: Box::new(Default::default()),
        signature: Box::new(Default::default()),
    };
    let boxx2 = dbb.save(&sig);
}

struct SecurityBoxRepo {}

impl BasicRepo<UserSecurityBox> for SecurityBoxRepo {
    fn save(self, entity: &UserSecurityBox) {
        todo!();
    }

    fn get(self) -> Result<UserSecurityBox, Error> {
        todo!();
    }
}

impl BasicRepo<UserSignature> for SecurityBoxRepo {
    fn save(self, entity: &UserSignature) {
        todo!()
    }

    fn get(self) -> Result<UserSignature, Error> {
        todo!()
    }
}
