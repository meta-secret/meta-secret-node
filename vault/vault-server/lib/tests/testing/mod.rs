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

pub mod framework {
    use async_std::task;
    use rocket::http::{ContentType, Status};
    use rocket::uri;

    use meta_secret_vault_server_lib::api::api::{
        JoinRequest, RegistrationResponse, RegistrationStatus, UserSignature,
    };
    use meta_secret_vault_server_lib::restful_api;
    use meta_secret_vault_server_lib::restful_api::membership::{
        MemberShipResponse, MembershipStatus,
    };

    use crate::MetaSecretDocker;

    pub struct Signatures {
        pub sig_1: UserSignature,
        pub sig_2: UserSignature,
        pub sig_3: UserSignature,
    }

    impl Default for Signatures {
        fn default() -> Self {
            let sig_1 = UserSignature::generate_default_for_tests();
            let sig_2 = UserSignature::generate_default_for_tests();
            let sig_3 = UserSignature::generate_default_for_tests();

            Signatures {
                sig_1,
                sig_2,
                sig_3,
            }
        }
    }

    pub struct MetaSecretTestApp {
        pub infra: MetaSecretDocker,
        pub signatures: Signatures,
    }

    impl MetaSecretTestApp {
        pub fn new(infra: MetaSecretDocker) -> Self {
            Self {
                infra,
                signatures: Signatures::default(),
            }
        }

        pub fn actions<A>(self, action: A)
        where
            A: Fn(&MetaSecretTestApp),
        {
            action(&self)
        }
    }

    pub struct TestAction<'a> {
        app: &'a MetaSecretTestApp,
    }

    impl<'a> TestAction<'a> {
        pub fn new(app: &'a MetaSecretTestApp) -> Self {
            Self { app }
        }

        pub fn register(self, user_sig: &UserSignature) -> RegistrationResponse {
            let signup_response = self
                .app
                .infra
                .rocket_client
                .post(uri!(restful_api::register::register))
                .header(ContentType::JSON)
                .body(serde_json::to_string_pretty(user_sig).unwrap())
                .dispatch();

            let signup_response = task::block_on(signup_response);
            assert_eq!(signup_response.status(), Status::Ok);

            let resp = signup_response.into_json::<RegistrationResponse>();
            let resp = task::block_on(resp);
            resp.unwrap()
        }

        pub fn accept(self, join_req: &JoinRequest) -> MemberShipResponse {
            let join_response = self
                .app
                .infra
                .rocket_client
                .post(uri!(restful_api::membership::accept))
                .header(ContentType::JSON)
                .body(serde_json::to_string_pretty(&join_req).unwrap())
                .dispatch();

            let join_response = task::block_on(join_response);
            assert_eq!(join_response.status(), Status::Ok);

            let resp = join_response.into_json::<MemberShipResponse>();
            let resp = task::block_on(resp);
            resp.unwrap()
        }

        pub fn create_cluster(&self) {
            let resp = TestAction::new(self.app).register(&self.app.signatures.sig_1);
            assert_eq!(resp.status, RegistrationStatus::Registered);

            let resp = TestAction::new(self.app).register(&self.app.signatures.sig_2);
            assert_eq!(resp.status, RegistrationStatus::AlreadyExists);

            let resp = TestAction::new(self.app).register(&self.app.signatures.sig_3);
            assert_eq!(resp.status, RegistrationStatus::AlreadyExists);

            let accept_resp = TestAction::new(self.app).accept(&JoinRequest {
                member: self.app.signatures.sig_1.clone(),
                candidate: self.app.signatures.sig_2.clone(),
            });
            assert_eq!(accept_resp.status, MembershipStatus::Finished);

            let accept_resp = TestAction::new(self.app).accept(&JoinRequest {
                member: self.app.signatures.sig_2.clone(),
                candidate: self.app.signatures.sig_3.clone(),
            });
            assert_eq!(accept_resp.status, MembershipStatus::Finished);
        }
    }
}
