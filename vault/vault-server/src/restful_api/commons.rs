use crate::crypto::crypto;
use crate::{Db, UserSignature, VaultDoc};
use mongodb::bson;
use rocket::State;

pub async fn find_vault(db: &State<Db>, user_sig: &UserSignature) -> Option<VaultDoc> {
    info!("verify: {:?}", user_sig);

    let is_valid = crypto::verify(&user_sig);
    if !is_valid {
        panic!("Can't pass signature verification");
    }

    let vault_filter = bson::doc! {
        "vaultName": user_sig.vault_name.clone()
    };

    let vaults_col = db.vaults_col();
    vaults_col.find_one(vault_filter, None).await.unwrap()
}
