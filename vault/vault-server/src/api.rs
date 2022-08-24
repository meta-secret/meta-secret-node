use serde::{Deserialize, Serialize};
use crate::db::VaultDoc;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserSignature {
    /// distributed vault, unique across entire system
    pub vault_name: String,
    pub device_name: String,

    pub public_key: String,
    pub rsa_public_key: String,

    /// Users' signature. Can be verified by:
    ///     ```signature == ed_dsa::verify(message: user_name, key: public_key)```
    pub signature: String,
}

impl UserSignature {
    pub fn to_initial_vault_doc(self) -> VaultDoc {
        VaultDoc {
            vault_name: self.vault_name.clone(),
            signatures: vec![self],
            pending_joins: vec![],
            declined_joins: vec![]
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
    Registered,
    AlreadyExists,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinRequest {
    pub member: UserSignature,
    pub candidate: UserSignature,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EncryptedMessage {
    /// Massage receiver who can decrypt message.
    /// Message text encrypted with receivers' RSA public key
    pub receiver: UserSignature,
    pub encrypted_text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VaultInfo {
    pub status: VaultInfoStatus,
    pub vault: Option<VaultDoc>,
}

impl VaultInfo {

    pub fn pending() -> Self {
        VaultInfo::empty(VaultInfoStatus::Pending)
    }

    pub fn declined() -> Self {
        VaultInfo::empty(VaultInfoStatus::Declined)
    }

    pub fn unknown() -> VaultInfo {
        VaultInfo::empty(VaultInfoStatus::Unknown)
    }

    pub fn empty(status: VaultInfoStatus) -> Self {
        VaultInfo { status, vault: None }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum VaultInfoStatus {
    /// Device is a member of a vault
    Member,
    /// Device is waiting to be added to a vault
    Pending,
    /// Vault members declined to add a device into the vault
    Declined,
    /// Device can't get any information about the vault, because its signature is not in members or pending list
    Unknown
}