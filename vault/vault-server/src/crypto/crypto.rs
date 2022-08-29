use crate::{DigitalSignatureRaw, UserSignature};

pub fn verify(user_signature: &UserSignature) -> bool {
    println!("Verify user signature");

    let sig = DigitalSignatureRaw::parse(user_signature);
    sig.transform().verify().is_ok()
}