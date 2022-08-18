use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRequest {
    pub user_name: String,
    pub public_key: String,
    pub signature_of_user_name: String,
}

impl UserRequest {
    pub fn to_user_doc(&self) -> UserDoc {
        UserDoc { user_name: self.user_name.clone(), public_key: self.public_key.clone() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterResponse {
    pub(crate) result: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDoc {
    user_name: String,
    public_key: String,
}
