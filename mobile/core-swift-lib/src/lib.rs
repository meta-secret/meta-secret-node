use meta_secret_core::shared_secret::shared_secret::PlainText;

#[no_mangle]
pub extern "C" fn hello_world() {
    let text = PlainText::from_str("hello world".to_string());
    println!("{:?}", text)
}