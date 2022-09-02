use ed25519_dalek::{PublicKey, Signature, SignatureError, Verifier};

use crate::UserSignature;

pub struct DigitalSignature {
    message: String,
    pub_key: PublicKey,
    sig: Signature,
}

pub struct DigitalSignatureRaw {
    message: String,
    pub_key: Vec<u8>,
    sig: Vec<u8>,
}

impl DigitalSignatureRaw {
    pub fn parse(user_sig: &UserSignature) -> DigitalSignatureRaw {
        let pub_key = &user_sig.public_key;
        let pub_key = base64::decode(pub_key).unwrap();

        let sig = user_sig.signature.as_bytes();
        let sig = base64::decode(sig).unwrap();

        DigitalSignatureRaw {
            message: user_sig.vault_name.clone(),
            pub_key,
            sig,
        }
    }

    pub fn transform(&self) -> DigitalSignature {
        DigitalSignature {
            message: self.message.clone(),
            pub_key: PublicKey::from_bytes(self.pub_key.as_slice()).unwrap(),
            sig: Signature::from_bytes(self.sig.as_slice()).unwrap(),
        }
    }
}

impl DigitalSignature {
    pub fn verify(&self) -> Result<(), SignatureError> {
        return self.pub_key.verify(self.message.as_bytes(), &self.sig);
    }
}
