use meta_secret_core::models::{MetaVault, UserCredentials};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DataEvent {
    pub key: Key,
    pub value: Value,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Key {
    pub ns: String,
    pub store: String,
    pub id: String,
}

pub struct MetaDb {
    pub meta_store: MetaStore,
}

pub struct DbLog {
    pub events: Vec<DataEvent>,
}

pub struct MetaStore {
    pub meta_vault: Option<MetaVault>,
    pub credentials: Option<UserCredentials>,
}

/// Read commit log from the database
pub fn load_commit_log() {}

/// Reads db log and transforms to the latest snapshot of the database
pub fn transform_commit_log(commit_log: &DbLog) -> MetaDb {
    let mut meta_db = MetaDb {
        meta_store: MetaStore {
            meta_vault: None,
            credentials: None,
        },
    };

    for event in &commit_log.events {
        let meta_vault_key = Key {
            ns: "meta_db".to_string(),
            store: "meta_schema".to_string(),
            id: "meta_vault".to_string(),
        };
        if event.key == meta_vault_key {
            let meta_vault = serde_json::from_value(event.value.clone()).unwrap();
            meta_db.meta_store.meta_vault = Some(meta_vault)
        }
    }

    meta_db
}

pub mod indexed_db {
    use async_trait::async_trait;
    use meta_secret_core::node::db::{FindAllQuery, GetCommand, SaveCommand};

    use crate::commit_log::DataEvent;
    use crate::db::WasmDbError;
    use crate::{idbFindAll, idbSave};

    pub struct CommitLogWasmRepo {
        pub db_name: String,
        pub store_name: String,
    }

    #[async_trait(? Send)]
    impl FindAllQuery<DataEvent> for CommitLogWasmRepo {
        type Error = WasmDbError;

        async fn find_all(&self) -> Result<Vec<DataEvent>, Self::Error> {
            let events_js = idbFindAll(self.db_name.as_str(), self.store_name.as_str()).await;
            let events: Vec<DataEvent> = serde_wasm_bindgen::from_value(events_js)?;
            Ok(events)
        }
    }

    #[async_trait(? Send)]
    impl SaveCommand<DataEvent> for CommitLogWasmRepo {
        type Error = WasmDbError;

        async fn save(&self, key: &str, event: &DataEvent) -> Result<(), Self::Error> {
            let event_js = serde_wasm_bindgen::to_value(event)?;
            idbSave(
                self.db_name.as_str(),
                self.store_name.as_str(),
                key,
                event_js,
            )
            .await;
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use meta_secret_core::models::MetaVault;
    use serde_json::Value;

    use crate::commit_log::indexed_db::CommitLogWasmRepo;
    use crate::commit_log::{transform_commit_log, DataEvent, DbLog};

    #[test]
    fn test_event() {
        let event_str = r#"{
            "key": {
                "ns": "meta_db",
                "store": "meta_schema",
                "id": "meta_vault"
            },
            "type": "MetaVault",
            "value": {
                "vaultName": "user1",
                "device": {
                    "deviceId": "123",
                    "deviceName": "my iphone"
                }
            }
        }"#;

        let json_val: Value = serde_json::from_str(event_str).unwrap();
        let value_type = json_val.get("type").unwrap().as_str().unwrap();

        if value_type == "MetaVault" {
            let meta_vault_val = json_val.get("value").unwrap();
            serde_json::from_value::<MetaVault>(meta_vault_val.clone()).unwrap();
        }

        let event: DataEvent = serde_json::from_str(event_str).unwrap();
        let db_log = DbLog {
            events: vec![event],
        };

        let db = transform_commit_log(&db_log);
        println!("{:?}", db.meta_store.meta_vault.unwrap());
    }
}
