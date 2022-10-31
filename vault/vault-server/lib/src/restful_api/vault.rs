use rocket::post;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::api::{UserSignature, VaultInfo, VaultInfoStatus};
use crate::restful_api::commons;
use crate::restful_api::commons::MetaState;

#[post("/getVault", format = "json", data = "<user_signature>")]
pub async fn get_vault(state: &State<MetaState>, user_signature: Json<UserSignature>) -> Json<VaultInfo> {
    let user_signature = user_signature.into_inner();

    let maybe_vault = commons::find_vault(&state.db, &user_signature).await;

    match maybe_vault {
        None => Json(VaultInfo::unknown()),
        Some(vault) => {
            if vault.signatures.contains(&user_signature) {
                return Json(VaultInfo {
                    status: VaultInfoStatus::Member,
                    vault: Some(vault),
                });
            }

            if vault.pending_joins.contains(&user_signature) {
                return Json(VaultInfo::pending());
            }

            if vault.declined_joins.contains(&user_signature) {
                return Json(VaultInfo::declined());
            }

            Json(VaultInfo::unknown())
        }
    }
}
