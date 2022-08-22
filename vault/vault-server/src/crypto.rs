use ed25519_dalek::{PublicKey, Signature, Verifier};
use tracing::info;

use crate::UserSignature;

pub fn verify(user_signature: &UserSignature) -> bool {
    println!("Verify user signature");

    let pub_key_bytes = user_signature.public_key.clone();
    let pub_key_bytes = base64::decode(pub_key_bytes).unwrap();
    let pub_key_bytes = pub_key_bytes.as_slice();
    let public_key: PublicKey = PublicKey::from_bytes(pub_key_bytes).unwrap();

    let signature_bytes = user_signature.signature.as_bytes();
    let signature_bytes = base64::decode(signature_bytes).unwrap();
    let signature_bytes = signature_bytes.as_slice();
    let signature: Signature = Signature::try_from(signature_bytes).unwrap();

    return public_key
        .verify(user_signature.vault_name.as_bytes(), &signature)
        .is_ok();
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use ed25519_dalek::{Keypair, Signer};
    use ed25519_dalek::ed25519::signature::Signature;
    use rand::rngs::OsRng;
    use rocket::serde::json::serde_json;

    use crate::crypto;
    use crate::UserSignature;

    use openssl::encrypt::{Encrypter, Decrypter};
    use openssl::rsa::{Rsa, Padding};
    use openssl::pkey::PKey;

    #[test]
    fn verify() {
        let mut csprng = OsRng {};
        let keypair: Keypair = Keypair::generate(&mut csprng);

        // Generate a keypair
        let rsa_keypair = Rsa::generate(4096).unwrap();
        let rsa_keypair = PKey::from_rsa(rsa_keypair).unwrap();
        let rsa_public_key = rsa_keypair.public_key_to_pem().unwrap();
        let rsa_public_key = String::from_utf8(rsa_public_key).unwrap();

        let vault_name = String::from("test_vault");
        let signature = base64::encode(keypair.sign(vault_name.as_bytes()).as_bytes());

        let user_signature = UserSignature {
            vault_name: vault_name.clone(),
            device_name: "test_device".to_string(),
            public_key: base64::encode(keypair.public.as_bytes()),
            rsa_public_key: rsa_public_key.clone(),
            signature: signature.clone(),
        };

        //let user_request_json_str = serde_json::to_string(&user_signature).unwrap();
        //println!("user request: {:?}", user_request_json_str);
        //let json_file = &File::create("user_request.json").unwrap();
        //serde_json::to_writer(json_file, &user_signature).unwrap();

        let is_valid = crypto::verify(&user_signature);
        assert!(is_valid);

        let user_request_json_str = serde_json::to_string(&user_signature).unwrap();
        println!("user request: {:?}", user_request_json_str);

        let invalid_user_request = UserSignature {
            vault_name: String::from("another_user"),
            device_name: "test_device".to_string(),
            public_key: base64::encode(keypair.public.as_bytes()),
            rsa_public_key: rsa_public_key.clone(),
            signature,
        };
        let is_valid = crypto::verify(&invalid_user_request);
        assert!(!is_valid);
    }
}