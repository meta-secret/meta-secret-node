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
        pub fixture: TestFixture,
    }

    impl Default for TestRunner {
        fn default() -> Self {
            Self {
                fixture: TestFixture::default(),
            }
        }
    }
}

pub mod test_infra {
    use async_trait::async_trait;
    use mongodb::Client;
    use rocket::local::asynchronous::Client as RocketClient;
    use testcontainers::clients::Cli;
    use testcontainers::images::mongo::Mongo;
    use testcontainers::Container;
    use tracing::info;

    use meta_secret_vault_server_lib::db::Db;
    use meta_secret_vault_server_lib::restful_api::commons::{get_server_key_manager, MetaState};
    use meta_secret_vault_server_lib::restful_api::meta_secret_routes;

    use crate::testing::testify::TestFixture;

    pub struct MetaSecretDocker {
        pub mongo_db_port: u16,
        pub mongo_db_url: String,
        pub rocket_client: RocketClient,
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
            let key_manager = get_server_key_manager(&db).await;

            let meta_state = MetaState { db, key_manager };

            let rocket = rocket::build().manage(meta_state).mount("/", meta_secret_routes());

            let rocket_client = RocketClient::tracked(rocket).await.expect("valid rocket instance");

            Self {
                mongo_db_port: container.get_host_port_ipv4(27017),
                mongo_db_url: format!("mongodb://localhost:{}/", host_port),
                rocket_client,
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
}

pub mod framework {
    use async_std::task;
    use meta_secret_core::crypto::keys::KeyManager;
    use rocket::http::{ContentType, Status};
    use rocket::{info, uri};

    use meta_secret_vault_server_lib::api::api::{
        JoinRequest, MessageStatus, PasswordRecoveryRequest, RegistrationResponse, RegistrationStatus, UserSignature,
        VaultInfo,
    };
    use meta_secret_vault_server_lib::db::SecretDistributionDoc;
    use meta_secret_vault_server_lib::restful_api;
    use meta_secret_vault_server_lib::restful_api::basic::MongoDbStats;
    use meta_secret_vault_server_lib::restful_api::membership::{MemberShipResponse, MembershipStatus};

    use crate::MetaSecretDocker;

    pub struct Signatures {
        pub key_manager_1: KeyManager,
        pub key_manager_2: KeyManager,
        pub key_manager_3: KeyManager,

        pub sig_1: UserSignature,
        pub sig_2: UserSignature,
        pub sig_3: UserSignature,
    }

    impl Default for Signatures {
        fn default() -> Self {
            let key_manager_1 = KeyManager::generate();
            let key_manager_2 = KeyManager::generate();
            let key_manager_3 = KeyManager::generate();

            let sig_1 = UserSignature::generate_default_for_tests(&key_manager_1);
            let sig_2 = UserSignature::generate_default_for_tests(&key_manager_2);
            let sig_3 = UserSignature::generate_default_for_tests(&key_manager_3);

            Signatures {
                key_manager_1,
                key_manager_2,
                key_manager_3,

                sig_1,
                sig_2,
                sig_3,
            }
        }
    }

    impl Signatures {
        pub fn all_signatures(&self) -> Vec<&UserSignature> {
            vec![&self.sig_1, &self.sig_2, &self.sig_3]
        }

        pub fn all_key_managers(&self) -> Vec<&KeyManager> {
            vec![&self.key_manager_1, &self.key_manager_2, &self.key_manager_3]
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

        pub fn stats(self) -> MongoDbStats {
            info!("Get Db statistics");

            let request = self
                .app
                .infra
                .rocket_client
                .get(uri!(restful_api::basic::stats))
                .header(ContentType::JSON)
                .dispatch();

            let response = task::block_on(request);
            assert_eq!(response.status(), Status::Ok);

            let resp = response.into_json::<MongoDbStats>();
            let resp = task::block_on(resp);
            resp.unwrap()
        }

        pub fn register(self, user_sig: &UserSignature) -> RegistrationResponse {
            info!(
                "Registering a new device, vault: {:?}, user pk: {:?}",
                user_sig.vault_name, user_sig.public_key.base64_text
            );

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
            info!(
                "Accept join request. A new device added into the vault/cluster: {}",
                join_req.candidate.public_key.base64_text
            );

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
            info!("Create meta secret cluster");

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

        pub fn distribute_password(&self, request: &SecretDistributionDoc) -> MessageStatus {
            info!("Distribute password: {:?}", request.meta_password.meta_password.id);

            let distribute_response = self
                .app
                .infra
                .rocket_client
                .post(uri!(restful_api::password::distribute))
                .header(ContentType::JSON)
                .body(serde_json::to_string_pretty(request).unwrap())
                .dispatch();

            let distribute_response = task::block_on(distribute_response);
            assert_eq!(distribute_response.status(), Status::Ok);

            let resp = distribute_response.into_json::<MessageStatus>();
            let resp = task::block_on(resp);
            let resp = resp.unwrap();

            assert_eq!(resp, MessageStatus::Ok);

            resp
        }

        pub fn get_vault(&self, sig: &UserSignature) -> VaultInfo {
            info!(
                "Get vault: {:?}, caller: {:?}",
                sig.vault_name, sig.public_key.base64_text
            );

            let resp = self
                .app
                .infra
                .rocket_client
                .post(uri!(restful_api::vault::get_vault))
                .header(ContentType::JSON)
                .body(serde_json::to_string_pretty(sig).unwrap())
                .dispatch();

            let resp = task::block_on(resp);
            assert_eq!(resp.status(), Status::Ok);

            let resp = resp.into_json::<VaultInfo>();
            let resp = task::block_on(resp);

            resp.unwrap()
        }

        pub fn find_shares(&self, sig: &UserSignature) -> Vec<SecretDistributionDoc> {
            info!("Find shares for {}", sig.public_key.base64_text);

            let resp = self
                .app
                .infra
                .rocket_client
                .post(uri!(restful_api::password::find_shares))
                .header(ContentType::JSON)
                .body(serde_json::to_string_pretty(sig).unwrap())
                .dispatch();

            let resp = task::block_on(resp);
            assert_eq!(resp.status(), Status::Ok);

            let resp = resp.into_json::<Vec<SecretDistributionDoc>>();
            let resp = task::block_on(resp);

            resp.unwrap()
        }

        pub fn claim_for_password_recovery(&self, recovery_request: &PasswordRecoveryRequest) -> MessageStatus {
            info!("claim_for_password_recovery");
            let resp = self
                .app
                .infra
                .rocket_client
                .post(uri!(restful_api::password::claim_for_password_recovery))
                .header(ContentType::JSON)
                .body(serde_json::to_string_pretty(recovery_request).unwrap())
                .dispatch();

            let resp = task::block_on(resp);
            assert_eq!(resp.status(), Status::Ok);

            let resp = resp.into_json::<MessageStatus>();
            let resp = task::block_on(resp);

            resp.unwrap()
        }

        pub async fn find_password_recovery_claims(
            &self,
            user_signature: &UserSignature,
        ) -> Vec<PasswordRecoveryRequest> {
            info!("find_password_recovery_claims");

            let resp = self
                .app
                .infra
                .rocket_client
                .post(uri!(restful_api::password::find_password_recovery_claims))
                .header(ContentType::JSON)
                .body(serde_json::to_string_pretty(user_signature).unwrap())
                .dispatch()
                .await;

            assert_eq!(resp.status(), Status::Ok);

            let resp = resp.into_json::<Vec<PasswordRecoveryRequest>>().await;

            resp.unwrap()
        }
    }
}
