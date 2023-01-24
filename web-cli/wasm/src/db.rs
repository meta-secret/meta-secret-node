use js_sys::Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{
    Event, IdbDatabase, IdbFactory, IdbOpenDbRequest, IdbRequest, IdbTransaction, Window,
};

use crate::log;

pub fn tx<T: AsRef<str>>(store_names: &[T], task: Box<dyn Fn(&IdbDatabase, &IdbTransaction)>) {
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

        task.as_ref()(db, &tx);

        tx.commit().unwrap();
    });

    open_db_request.set_onsuccess(Some(on_success_action.as_ref().unchecked_ref()));

    //todo fix memory leaks (see wasm_bindgen::Closure doc and https://rustwasm.github.io/wasm-bindgen/reference/passing-rust-closures-to-js.html)
    on_success_action.forget();
    on_upgrade_action.forget();
}
