use mongodb::bson;
use rocket::serde::json::Json;
use rocket::State;

use crate::{Db, EncryptedMessage, SecretDistributionDoc, StreamExt, UserSignature};

#[post("/distribute", format = "json", data = "<encrypted_password_share>")]
pub async fn distribute(db: &State<Db>, encrypted_password_share: Json<EncryptedMessage>) -> Json<String> {
    let secrets_distribution_col = db.distribution_col();

    //create a new user:
    let record = SecretDistributionDoc {
        secret_message: encrypted_password_share.into_inner()
    };

    secrets_distribution_col.insert_one(record, None)
        .await
        .unwrap();

    Json("OK".to_string())
}

#[post("/findShares", format = "json", data = "<user_signature>")]
pub async fn find_shares(db: &State<Db>, user_signature: Json<UserSignature>) -> Json<Vec<SecretDistributionDoc>> {
    let secrets_distribution_col = db.distribution_col();

    //find shares
    let secret_shares_filter = bson::doc! {
        "secret_message.receiver.rsa_public_key": user_signature.into_inner().rsa_public_key.clone()
    };

    let mut shares_docs = secrets_distribution_col
        .find(secret_shares_filter, None)
        .await
        .unwrap();

    let mut shares = vec![];
    while let Some(share) = shares_docs.next().await {
        shares.push(share.unwrap());
    }

    Json(shares)
}
