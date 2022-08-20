use serde::{Deserialize, Serialize};
use crate::db::VaultDoc;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserSignature {
    /// distributed vault, unique across entire system
    pub vault_name: String,

    pub public_key: String,
    pub rsa_public_key: String,

    /// Users' signature. Can be verified by:
    ///     ```signature == ed_dsa::verify(message: user_name, key: public_key)```
    pub signature: String
}

impl UserSignature {
    pub fn to_initial_vault_doc(self) -> VaultDoc {
        VaultDoc {
            vault_name: self.vault_name.clone(),
            signatures: vec![self],
            pending_joins: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationResponse {
    pub status: RegistrationStatus,
    pub result: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RegistrationStatus {
    Registered, AlreadyExists
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinRequest {
    pub member: UserSignature,
    pub candidate: UserSignature
}