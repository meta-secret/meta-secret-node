use mongodb::bson;
use rocket::serde::json::Json;
use rocket::State;

use crate::{Db, UserSignature, VaultDoc, VaultInfo, VaultInfoStatus};

#[post("/getVault", format = "json", data = "<user_signature>")]
pub async fn get_vault(db: &State<Db>, user_signature: Json<UserSignature>) -> Json<VaultInfo> {
    let user_signature = user_signature.into_inner();

    let vaults_col = db.vaults_col();
    let vaults_filter = bson::doc! {
        "vaultName": user_signature.vault_name.clone()
    };

    let maybe_vault: Option<VaultDoc> = vaults_col
        .find_one(vaults_filter, None)
        .await
        .unwrap();

    return match maybe_vault {
        None => {
            Json(VaultInfo::unknown())
        }
        Some(vault) => {
            if vault.signatures.contains(&user_signature) {
                return Json(VaultInfo { status: VaultInfoStatus::Member, vault: Some(vault) });
            }

            if vault.pending_joins.contains(&user_signature) {
                return Json(VaultInfo::pending());
            }

            if vault.declined_joins.contains(&user_signature) {
                return Json(VaultInfo::declined());
            }

            Json(VaultInfo::unknown())
        }
    };
}
