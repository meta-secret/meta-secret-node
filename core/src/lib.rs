pub mod shared_secret;

use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::ops::Not;
use std::path::Path;
use crate::shared_secret::data_block::common::SharedSecretConfig;
use crate::shared_secret::data_block::shared_secret_data_block::SharedSecretBlock;
use crate::shared_secret::shared_secret::{
    PlainText, SharedSecret, SharedSecretEncryption, UserShareDto,
};

use image;
use rqrr;

pub fn restore() -> PlainText {
    //read json files
    let shares = fs::read_dir("secrets").unwrap();

    let mut users_shares_dto: Vec<UserShareDto> = vec![];
    for secret_share_file in shares {
        let file_path = secret_share_file.unwrap().path();

        let extension = file_path
            .extension()
            .and_then(OsStr::to_str)
            .unwrap();

        if !extension.eq("json") {
            continue;
        }

        // Open the file in read-only mode with buffer.
        let file = File::open(file_path)
            .expect("Unable to open file");
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `User`.
        let secret_share: UserShareDto = serde_json::from_reader(reader).unwrap();
        users_shares_dto.push(secret_share);
    }

    let mut secret_blocks: Vec<SharedSecretBlock> = vec![];

    let blocks_num: usize = users_shares_dto[0].share_blocks.len();

    for block_index in 0..blocks_num {
        let mut encrypted_data_blocks = vec![];

        for user_share in users_shares_dto.iter() {
            let encrypted_data_block = user_share.get_encrypted_data_block(block_index);
            encrypted_data_blocks.push(encrypted_data_block);
        }

        let curr_block = &users_shares_dto[0].share_blocks[block_index];
        let secret_block = SharedSecretBlock {
            config: curr_block.config,
            meta_data: curr_block.meta_data.clone(),
            shares: encrypted_data_blocks,
        };

        secret_blocks.push(secret_block);
    }

    let secret = SharedSecret {
        secret_blocks
    };

    secret.restore()
}

pub fn split(secret: String, config: SharedSecretConfig) {
    let plain_text = PlainText::from_str(secret);
    let shared_secret = SharedSecretEncryption::new(config, &plain_text);

    fs::create_dir_all("secrets").unwrap();

    for share_index in 0..config.number_of_shares {
        let share: UserShareDto = shared_secret.get_share(share_index);
        let share_json = serde_json::to_string_pretty(&share).unwrap();

        // Save the JSON structure into the output file
        fs::write(
            format!("secrets/shared-secret-{share_index}.json"),
            share_json.clone(),
        ).unwrap();

        //generate qr code
        generate_qr_code(
            share_json.as_str(),
            format!("secrets/shared-secret-{share_index}.png").as_str(),
        )
    }
}

pub fn generate_qr_code(data: &str, path: &str) {
    use qrcode_generator::QrCodeEcc;

    qrcode_generator::to_png_to_file(data, QrCodeEcc::High, data.len(), path)
        .unwrap();
}

pub fn convert_qr_images_to_json_files() {
    let shares = fs::read_dir("secrets").unwrap();

    let mut share_index = 0;
    for secret_share_file in shares {
        let file_path = secret_share_file.unwrap().path();

        let extension = file_path
            .extension()
            .and_then(OsStr::to_str)
            .unwrap();

        if !extension.eq("png") {
            continue;
        }

        let json_str = read_qr_code(file_path.as_path());
        fs::write(
            format!("secrets/shared-secret-{share_index}.json"),
            json_str,
        ).unwrap();

        share_index += 1;
    }
}

pub fn read_qr_code(path: &Path) -> String {
    let img = image::open(path).unwrap().to_luma8();
    // Prepare for detection
    let mut img = rqrr::PreparedImage::prepare(img);
    // Search for grids, without decoding
    let grids = img.detect_grids();
    assert_eq!(grids.len(), 1);
    // Decode the grid
    let (meta, content) = grids[0].decode().unwrap();
    return content;
}