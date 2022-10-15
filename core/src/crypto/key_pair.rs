use crypto_box::{
    aead::{Aead, OsRng as CryptoBoxOsRng, Payload},
    ChaChaBox, Nonce, PublicKey as CryptoBoxPublicKey, SecretKey as CryptoBoxSecretKey,
};
use ed25519_dalek::ed25519::signature::Signature;
use ed25519_dalek::{Keypair, Signer};
use image::EncodableLayout;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use crate::crypto::encoding::Base64EncodedText;
use crate::crypto::keys::{AeadCipherText, AeadPlainText};

pub trait KeyPair {
    fn generate() -> Self;
    fn public_key(&self) -> Base64EncodedText;
}

pub struct DsaKeyPair {
    pub key_pair: Keypair,
}

impl DsaKeyPair {
    pub fn sign(&self, text: String) -> Base64EncodedText {
        let signature = self.key_pair.sign(text.as_bytes());
        Base64EncodedText::from(signature.as_bytes())
    }
}

impl KeyPair for DsaKeyPair {
    fn generate() -> Self {
        let mut cs_prng = OsRng {};
        let key_pair = Keypair::generate(&mut cs_prng);

        DsaKeyPair { key_pair }
    }

    fn public_key(&self) -> Base64EncodedText {
        Base64EncodedText::from(&self.key_pair.public.to_bytes())
    }
}

pub struct TransportDsaKeyPair {
    pub secret_key: CryptoBoxSecretKey,
    pub public_key: CryptoBoxPublicKey,
}

impl KeyPair for TransportDsaKeyPair {
    fn generate() -> Self {
        let secret_key = CryptoBoxSecretKey::generate(&mut CryptoBoxOsRng);
        let public_key = secret_key.public_key();

        Self {
            secret_key,
            public_key,
        }
    }

    fn public_key(&self) -> Base64EncodedText {
        Base64EncodedText::from(self.public_key.as_bytes())
    }
}

impl TransportDsaKeyPair {
    pub fn build_cha_cha_box(&self, their_pk: &CryptoBoxPublicKey) -> ChaChaBox {
        ChaChaBox::new(their_pk, &self.secret_key)
    }

    pub fn encrypt(&self, plain_text: &AeadPlainText) -> AeadCipherText {
        let auth_data = &plain_text.auth_data;
        let receiver_pk = CryptoBoxPublicKey::from(&auth_data.receiver_public_key);
        let crypto_box = self.build_cha_cha_box(&receiver_pk);
        let cipher_text = crypto_box
            .encrypt(
                &Nonce::from(&auth_data.nonce),
                Payload {
                    msg: plain_text.msg.clone().as_bytes(), // your message to encrypt
                    aad: auth_data.associated_data.as_bytes(), // not encrypted, but authenticated in tag
                },
            )
            .unwrap();

        AeadCipherText {
            msg: Base64EncodedText::from(cipher_text),
            auth_data: plain_text.auth_data.clone(),
        }
    }

    pub fn decrypt(self, cipher_text: &AeadCipherText) -> AeadPlainText {
        let auth_data = &cipher_text.auth_data;
        let sender_pk = CryptoBoxPublicKey::from(&auth_data.sender_public_key);
        let crypto_box = self.build_cha_cha_box(&sender_pk);

        let msg_vec: Vec<u8> = cipher_text.msg.clone().into();
        let decrypted_plaintext: Vec<u8> = crypto_box
            .decrypt(
                &Nonce::from(&auth_data.nonce),
                Payload {
                    msg: msg_vec.as_bytes(),
                    aad: auth_data.associated_data.as_bytes(),
                },
            )
            .unwrap();

        AeadPlainText {
            msg: String::from_utf8(decrypted_plaintext).unwrap(),
            auth_data: cipher_text.auth_data.clone(),
        }
    }
}

#[cfg(test)]
pub mod test {
    use crypto_box::{
        aead::{AeadCore, OsRng},
        ChaChaBox, Nonce,
    };

    use crate::crypto::encoding::Base64EncodedText;
    use crate::crypto::key_pair::KeyPair;
    use crate::crypto::keys::{AeadAuthData, AeadCipherText, AeadPlainText, KeyManager};

    #[test]
    fn crypto_box() {
        let alice_km = KeyManager::generate();
        let bob_km = KeyManager::generate();

        let nonce: Nonce = ChaChaBox::generate_nonce(&mut OsRng);

        let plain_text = AeadPlainText {
            msg: "t0p$3cr3t".to_string(),
            auth_data: AeadAuthData {
                associated_data: "checksum".to_string(),
                sender_public_key: alice_km.transport_key_pair.public_key(),
                receiver_public_key: bob_km.transport_key_pair.public_key(),
                nonce: Base64EncodedText::from(nonce.as_slice()),
            },
        };
        let cipher_text: AeadCipherText = alice_km.transport_key_pair.encrypt(&plain_text);

        let decrypted_text = bob_km.transport_key_pair.decrypt(&cipher_text);

        assert_eq!(plain_text, decrypted_text);
    }
}