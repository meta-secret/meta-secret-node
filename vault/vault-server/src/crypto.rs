use ed25519_dalek::{PublicKey, Signature, Verifier};
use tracing::info;

use crate::UserRequest;

pub fn verify(user_request: &UserRequest) -> bool {
    println!("Verify user signature");

    let pub_key_bytes = user_request.public_key.clone();
    let pub_key_bytes = base64::decode(pub_key_bytes).unwrap();
    let pub_key_bytes = pub_key_bytes.as_slice();
    let public_key: PublicKey = PublicKey::from_bytes(pub_key_bytes).unwrap();

    let signature_bytes = user_request.signature_of_user_name.as_bytes();
    let signature_bytes = base64::decode(signature_bytes).unwrap();
    let signature_bytes = signature_bytes.as_slice();
    let signature: Signature = Signature::try_from(signature_bytes).unwrap();

    return public_key
        .verify(user_request.user_name.as_bytes(), &signature)
        .is_ok();
}

#[cfg(test)]
mod test {
    use ed25519_dalek::{Keypair, Signer};
    use ed25519_dalek::ed25519::signature::Signature;
    use rand::rngs::OsRng;
    use rocket::serde::json::serde_json;

    use crate::crypto::verify;
    use crate::UserRequest;

    #[test]
    fn verify_valid_sign() {
        let mut csprng = OsRng {};
        let keypair: Keypair = Keypair::generate(&mut csprng);

        let user_name = String::from("test_user");
        let signature = base64::encode(keypair.sign(user_name.as_bytes()).as_bytes());

        let user_request = UserRequest {
            user_name: user_name.clone(),
            public_key: base64::encode(keypair.public.as_bytes()),
            signature_of_user_name: signature.clone(),
        };

        let user_request = UserRequest {
            user_name: String::from("test_user"),
            public_key: String::from("922JB+F8ktWuQxeHWzlHZ3XH3/5/2EGma0aHa4Yu1FU="),
            signature_of_user_name: String::from("c92vK/pMACBEZKV76DSirQuw38PcDcOYjBrotVM00x35AhwWrW4POLhdh3+Ssaw0Wg8pUL1EWSY6+2WjbCNiDA=="),
        };


        let is_valid = verify(&user_request);
        assert!(is_valid);

        let user_request_json_str = serde_json::to_string(&user_request).unwrap();
        println!("user request: {:?}", user_request_json_str);

        let invalid_user_request = UserRequest {
            user_name: String::from("another_user"),
            public_key: base64::encode(keypair.public.as_bytes()),
            signature_of_user_name: signature,
        };
        let is_valid = verify(&invalid_user_request);
        assert!(!is_valid);
    }
}