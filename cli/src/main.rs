use std::fs::File;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

use meta_secret_core::{restore, split};
use meta_secret_core::shared_secret::data_block::common::SharedSecretConfig;

#[derive(Debug, Parser)]
#[clap(about = "Meta Secret Command Line Application", long_about = None)]
struct CmdLine {
    #[clap(subcommand)]
    command: Command,
}

/// Simple program to greet a person
#[derive(Subcommand, Debug)]
enum Command {
    Split {
        secret: String
    },
    Restore {

    },
}

#[derive(Debug, Serialize, Deserialize)]
struct MetaSecretConfig {
    shared_secret: SharedSecretConfig,
}

///https://kerkour.com/rust-cross-compilation
fn main() {
    let args: CmdLine = CmdLine::parse();

    let config_file: File = File::open("config.yaml").unwrap();
    let app_config: MetaSecretConfig = serde_yaml::from_reader(config_file).unwrap();
    let shared_secret_config = app_config.shared_secret;

    match args.command {
        Command::Split { secret } => {
            split(secret, shared_secret_config);
        }
        Command::Restore {} => {
            let text = restore();
            println!("Restored: {:?}", String::from_utf8(text.text).unwrap());
        }
    }

    println!("Finished")
}
