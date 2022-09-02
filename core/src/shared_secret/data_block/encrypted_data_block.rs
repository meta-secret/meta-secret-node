use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::shared_secret::data_block::common;
use crate::shared_secret::data_block::common::{BlockMetaData, DataBlockParserError};

pub const SECRET_DATA_BLOCK_SIZE: usize = 113;

//block of data after converting PlainDataBlock to a shared secret
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptedDataBlock {
    #[serde(with = "BigArray")]
    pub data: [u8; SECRET_DATA_BLOCK_SIZE],
}

impl EncryptedDataBlock {
    pub fn from_bytes(
        meta_data: &BlockMetaData,
        data: &[u8],
    ) -> Result<Self, DataBlockParserError> {
        // An array can't be empty
        if data == [0; SECRET_DATA_BLOCK_SIZE] {
            return Err(DataBlockParserError::Invalid);
        }

        if meta_data.size <= 0 || meta_data.size > SECRET_DATA_BLOCK_SIZE {
            return Err(DataBlockParserError::WrongSize);
        }

        let share = Self {
            data: common::parse_data::<SECRET_DATA_BLOCK_SIZE>(data),
        };

        return Ok(share);
    }

    pub fn from_base64(meta_data: &BlockMetaData, base64_data: String) -> EncryptedDataBlock {
        let data = base64::decode(base64_data).unwrap();
        let data = data.as_slice();

        EncryptedDataBlock::from_bytes(meta_data, data).unwrap()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_secret_data_block() {
        /*let secret = DataBlockShare::new(
            BlockMetaData { size: 3 },
            &[42; SECRET_DATA_BLOCK_SIZE],
        );
        println!("{:?}", secret);*/
    }
}
