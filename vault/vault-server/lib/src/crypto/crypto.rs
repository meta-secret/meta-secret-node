use crate::api::api::UserSignature;
use crate::crypto::digital_signature::DigitalSignatureRaw;

pub fn verify(user_signature: &UserSignature) -> bool {
    let sig = DigitalSignatureRaw::parse(user_signature);
    sig.transform().verify().is_ok()
}
