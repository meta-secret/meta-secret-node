use async_trait::async_trait;
use meta_secret_core::crypto::encoding::serialized_key_manager::SerializedKeyManager;
use meta_secret_core::crypto::keys::KeyManager;
use mongodb::Client;
use rocket::local::asynchronous::Client as RocketClient;
use testcontainers::clients::Cli;
use testcontainers::images::mongo::Mongo;
use testcontainers::Container;
use tracing::info;

use crate::testify::TestFixture;
use meta_secret_vault_server_lib::db::Db;
use meta_secret_vault_server_lib::restful_api::commons::{get_server_key_manager, MetaState};
use meta_secret_vault_server_lib::restful_api::meta_secret_routes;

pub struct MetaSecretDocker {
    pub mongo_db_port: u16,
    pub mongo_db_url: String,
    pub rocket_client: RocketClient,
    pub key_manager: SerializedKeyManager,
    pub db: Db,
}

#[async_trait(?Send)]
pub trait MetaSecretDockerInfra {
    async fn run(ctx: &TestFixture, docker_cli: &Cli, container: &Container<Mongo>) -> Self;
}

#[async_trait(?Send)]
impl MetaSecretDockerInfra for MetaSecretDocker {
    async fn run(ctx: &TestFixture, _docker_cli: &Cli, container: &Container<Mongo>) -> Self {
        let _ = pretty_env_logger::try_init();

        let host_port = container.get_host_port_ipv4(27017);
        let url = format!("mongodb://localhost:{}/", host_port);

        let mongo_db_client: Client = Client::with_uri_str(&url).await.unwrap();
        let mongo_db = mongo_db_client.database(ctx.db_schema.db_name.as_str());

        let db = Db {
            db_schema: ctx.clone().db_schema,
            url,
            client: mongo_db_client,
            db: mongo_db,
        };

        let key_manager: KeyManager = get_server_key_manager(&db).await;
        let serialized_km = SerializedKeyManager::from(&key_manager);

        let meta_state = MetaState {
            db: db.clone(),
            key_manager,
        };

        let rocket = rocket::build().manage(meta_state).mount("/", meta_secret_routes());

        let rocket_client = RocketClient::tracked(rocket).await.expect("valid rocket instance");

        Self {
            mongo_db_port: container.get_host_port_ipv4(27017),
            mongo_db_url: format!("mongodb://localhost:{}/", host_port),
            rocket_client,
            key_manager: serialized_km,
            db,
        }
    }
}

impl MetaSecretDocker {
    pub fn init_logging() {
        let tracing = tracing_subscriber::fmt()
            .compact()
            // enable everything
            .with_max_level(tracing::Level::DEBUG)
            // sets this to be the default, global collector for this application.
            .try_init();

        if tracing.is_err() {
            info!("Tracing already initialized");
        }

        info!("Meta Secret Infra!");
    }
}
