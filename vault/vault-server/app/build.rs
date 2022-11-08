use std::path::Path;
use std::{env, fs};

fn main() {
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

    copy_config(build_dir, "Rocket.toml");
    copy_config(build_dir, "mongodb-config.json");
}

fn copy_config(build_dir: &str, config_file: &str) {
    println!("cargo:warning=copy {config} into build directory", config = config_file);
    let mut config_dest_dir = build_dir.to_string();
    config_dest_dir.push('/');
    config_dest_dir.push_str(config_file);

    println!(
        "cargo:warning=output directory: {config}",
        config = config_dest_dir.as_str()
    );
    fs::copy(config_file, config_dest_dir.as_str()).unwrap_or_else(|_| panic!("Error copying {} config", config_file));
}
