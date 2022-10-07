pub mod testify {
    use meta_secret_vault_server_lib::db::DbSchema;

    #[derive(Clone, Debug)]
    pub struct TestFixture {
        pub db_schema: DbSchema,
    }

    impl Default for TestFixture {
        fn default() -> Self {
            Self {
                db_schema: DbSchema::default(),
            }
        }
    }

    pub struct TestRunner {
        fixture: TestFixture,
    }

    impl Default for TestRunner {
        fn default() -> Self {
            Self {
                fixture: TestFixture::default(),
            }
        }
    }

    impl TestRunner {
        pub fn run(self, action: fn(fixture: TestFixture) -> ()) {
            action(self.fixture)
        }
    }
}

pub mod test_infra {
    use async_trait::async_trait;
    use mongodb::Client;
    use rocket::local::asynchronous::Client as RocketClient;
    use rocket::routes;
    use testcontainers::clients::Cli;
    use testcontainers::images::mongo::Mongo;
    use testcontainers::Container;

    use meta_secret_vault_server_lib::db::Db;
    use meta_secret_vault_server_lib::restful_api;

    use crate::testing::testify::TestFixture;

    pub struct MetaSecretDocker {
        pub mongo_db_port: u16,
        pub mongo_db_url: String,
        pub db: Db,
        pub rocket_client: RocketClient,
    }

    #[async_trait(?Send)]
    pub trait MetaSecretDockerInfra {
        async fn run(ctx: &TestFixture, docker_cli: &Cli, container: &Container<Mongo>) -> Self;
    }

    #[async_trait(?Send)]
    impl MetaSecretDockerInfra for MetaSecretDocker {
        async fn run(ctx: &TestFixture, docker_cli: &Cli, container: &Container<Mongo>) -> Self {
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

            let routes = routes![
                restful_api::password::add_meta_password,
                restful_api::register::register,
                restful_api::membership::accept
            ];

            let rocket = rocket::build().manage(db.clone()).mount("/", routes);

            let rocket_client = RocketClient::tracked(rocket)
                .await
                .expect("valid rocket instance");

            Self {
                mongo_db_port: container.get_host_port_ipv4(27017),
                mongo_db_url: format!("mongodb://localhost:{}/", host_port),
                db,
                rocket_client,
            }
        }
    }
}
