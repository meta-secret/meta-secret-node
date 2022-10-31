use meta_secret_core::crypto::key_pair::KeyPair;
use rocket::serde::json::Json;
use rocket::{post, State};

use crate::api::api::{RegistrationResponse, UserSignature};
use crate::restful_api::commons::MetaState;
use crate::restful_api::register::register;

#[post("/joinMetaCloud", format = "json", data = "<register_request>")]
pub async fn join_meta_cloud(
    state: &State<MetaState>,
    register_request: Json<UserSignature>,
) -> Json<RegistrationResponse> {
    let register_request = register_request.into_inner();

    //todo!("Check that user is a member of the vault");

    let vault_name = register_request.vault_name;
    let user_sig = UserSignature {
        vault_name: vault_name.clone(),
        device: register_request.device,
        public_key: state.key_manager.dsa.public_key(),
        transport_public_key: state.key_manager.transport_key_pair.public_key(),
        signature: state.key_manager.dsa.sign(vault_name),
    };
    register(state, Json(user_sig)).await
}
