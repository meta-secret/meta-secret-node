use meta_secret_core::shared_secret::shared_secret::PlainText;

/// https://betterprogramming.pub/from-rust-to-swift-df9bde59b7cd

#[no_mangle]
pub extern "C" fn hello_world() {
    let text = PlainText::from_str("hello world".to_string());
    println!("{:?}", text)
}