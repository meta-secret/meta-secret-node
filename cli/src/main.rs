extern crate core;

use std::fs::File;

use clap::{Parser, Subcommand, ArgEnum};
use serde::{Deserialize, Serialize};

use meta_secret_core::{convert_qr_images_to_json_files, recover, split};
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
        #[clap(short, long)]
        secret: String
    },
    Restore {
        #[clap(short, long, arg_enum)]
        from: RestoreType
    },
}

#[derive(Debug, Clone, ArgEnum, Eq, PartialEq)]
#[clap(rename_all = "kebab_case")]
enum RestoreType {
    Qr, Json
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
        Command::Restore {from} => {
            match from {
                RestoreType::Qr => {
                    convert_qr_images_to_json_files();
                    restore_from_json();
                }
                RestoreType::Json => {
                    restore_from_json();
                }
            }
        }
    }

    println!("Finished")
}

fn restore_from_json() {
    let text = recover().unwrap();
    println!("Restored: {:?}", String::from_utf8(text.text).unwrap());
}
