use ed25519_dalek::ed25519::signature::Signature;
use ed25519_dalek::{Keypair, Signer};
use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
use rand::rngs::OsRng;

pub struct KeyManager {
    pub rsa: RsaKeyPair,
    pub dsa: DsaKeyPair,
}

pub struct RsaKeyPair {
    pub keypair: PKey<Private>,
}

pub struct DsaKeyPair {
    pub keypair: Keypair,
}

pub trait KeyPair {
    fn generate() -> Self;
    fn generate_default_for_tests() -> Self;

    fn public_key_serialized(&self) -> String;
}

impl KeyPair for RsaKeyPair {
    fn generate() -> RsaKeyPair {
        let keypair = Rsa::generate(4096).unwrap();
        let keypair = PKey::from_rsa(keypair).unwrap();

        RsaKeyPair { keypair }
    }

    fn generate_default_for_tests() -> Self {
        let keypair = Rsa::generate(0 as u32).unwrap();
        let keypair = PKey::from_rsa(keypair).unwrap();

        RsaKeyPair { keypair }
    }

    fn public_key_serialized(&self) -> String {
        let pub_key = self.keypair.public_key_to_pem().unwrap();
        String::from_utf8(pub_key).unwrap()
    }
}

impl KeyPair for DsaKeyPair {
    fn generate() -> Self {
        let mut cs_prng = OsRng {};
        let keypair = Keypair::generate(&mut cs_prng);

        DsaKeyPair { keypair }
    }

    fn generate_default_for_tests() -> Self {
        let bytes = [1; 64];
        let keypair = Keypair::from_bytes(&bytes).unwrap();
        DsaKeyPair { keypair }
    }

    fn public_key_serialized(&self) -> String {
        base64::encode(self.keypair.public.to_bytes())
    }
}

impl DsaKeyPair {
    pub fn sign(&self, text: &[u8]) -> String {
        let sign = self.keypair.sign(text);
        let sign = sign.as_bytes();
        base64::encode(sign)
    }
}

impl KeyManager {
    pub fn generate() -> KeyManager {
        KeyManager {
            rsa: RsaKeyPair::generate(),
            dsa: DsaKeyPair::generate(),
        }
    }

    pub fn generate_default_for_tests() -> KeyManager {
        KeyManager {
            rsa: RsaKeyPair::generate_default_for_tests(),
            dsa: DsaKeyPair::generate_default_for_tests(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::crypto::keys::{DsaKeyPair, KeyPair};
    use ed25519_dalek::Signer;

    #[test]
    fn test_dsa() {
        let key_pair = DsaKeyPair::generate();
        let _pub_key = key_pair.public_key_serialized();

        let msg = "yay".as_bytes();
        let signed = key_pair.keypair.sign(msg);

        let is_valid = key_pair.keypair.verify(msg, &signed).is_ok();

        assert!(is_valid)
    }
}
