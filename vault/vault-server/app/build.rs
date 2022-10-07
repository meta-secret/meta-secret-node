use std::path::Path;
use std::{env, fs};

fn main() {
    let rocket_config = "Rocket.toml";

    println!(
        "cargo:warning=copy {config} into build directory",
        config = rocket_config
    );
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
    config_dest_dir.push_str("/");
    config_dest_dir.push_str(rocket_config);

    println!(
        "cargo:warning=output directory: {config}",
        config = config_dest_dir.as_str()
    );
    fs::copy(rocket_config, config_dest_dir.as_str()).expect("Error copying config file");
}
