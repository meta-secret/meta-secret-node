use crate::api::api::UserSignature;
use crate::crypto::crypto;
use crate::db::{Db, VaultDoc};
use mongodb::bson;
use rocket::get;
use rocket::State;

pub const MAIN_MESSAGE: &str = "Hello Meta World!";

#[get("/")]
pub async fn hi() -> String {
    String::from(MAIN_MESSAGE)
}

pub async fn find_vault(db: &State<Db>, user_sig: &UserSignature) -> Option<VaultDoc> {
    let is_valid = crypto::verify(user_sig);
    if !is_valid {
        panic!("Can't pass signature verification");
    }

    let vault_filter = bson::doc! {
        "vaultName": user_sig.vault_name.clone()
    };

    let vaults_col = db.vaults_col();
    vaults_col.find_one(vault_filter, None).await.unwrap()
}
