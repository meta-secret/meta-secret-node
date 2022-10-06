use std::{env, fs};
use std::path::Path;

fn main() {
    println!("cargo:warning=copy config.yaml into build directory");
    let out_dir = env::var("OUT_DIR").unwrap();
    let build_dir = Path::new(out_dir.as_str())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap();

    let mut config_dest_dir = build_dir.to_string();
    config_dest_dir.push_str("/config.yaml");

    println!("cargo:warning=output directory: {:?}", config_dest_dir.as_str());
    fs::copy("config.yaml", config_dest_dir.as_str())
        .expect("Error copying config.yaml");
}