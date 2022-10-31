use mongodb::bson::doc;
use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;

use crate::restful_api::commons::MetaState;

pub const MAIN_MESSAGE: &str = "Hello Meta World!";

#[get("/")]
pub async fn hi() -> String {
    String::from(MAIN_MESSAGE)
}

#[get("/stats")]
pub async fn stats(state: &State<MetaState>) -> Json<MongoDbStats> {
    let connection = state.db.db.run_command(doc! {"ping": 1}, None).await.is_ok();
    let registrations = state.db.vaults_col().count_documents(None, None).await.unwrap_or(0);
    let stats = MongoDbStats {
        connection,
        registrations: registrations as usize,
    };

    Json(stats)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MongoDbStats {
    pub connection: bool,
    pub registrations: usize,
}
