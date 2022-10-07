use async_std::task;
use rocket::http::{ContentType, Status};
use rocket::uri;
use testcontainers::clients::Cli;
use testcontainers::images::mongo::Mongo;
use testcontainers::{clients, Container};

use meta_secret_vault_server_lib::api::api::{
    JoinRequest, RegistrationResponse, RegistrationStatus, UserSignature,
};
use meta_secret_vault_server_lib::restful_api;
use meta_secret_vault_server_lib::restful_api::membership::{MemberShipResponse, MembershipStatus};

use crate::testing::test_infra::{MetaSecretDocker, MetaSecretDockerInfra};
use crate::testing::testify::TestRunner;

mod testing;

struct Signatures {
    sig_1: UserSignature,
    sig_2: UserSignature,
    sig_3: UserSignature,
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

struct MetaSecretTestApp {
    infra: MetaSecretDocker,
    signatures: Signatures,
}

impl MetaSecretTestApp {
    fn new(infra: MetaSecretDocker) -> Self {
        Self {
            infra,
            signatures: Signatures::default(),
        }
    }

    fn actions<A>(self, action: A)
    where
        A: Fn(&MetaSecretTestApp),
    {
        action(&self)
    }
}

struct TestAction<'a> {
    app: &'a MetaSecretTestApp,
}

impl<'a> TestAction<'a> {
    fn new(app: &'a MetaSecretTestApp) -> Self {
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
}

#[rocket::async_test]
async fn register_one_device() {
    TestRunner::default().run(|ctx| {
        let docker_cli: Cli = clients::Cli::default();
        let container: Container<Mongo> = docker_cli.run(Mongo::default());

        let infra = MetaSecretDocker::run(&ctx, &docker_cli, &container);
        let infra = task::block_on(infra);
        println!("{:?}", infra.mongo_db_url);

        let test_app = MetaSecretTestApp::new(infra);

        test_app.actions(|app| {
            let user_sig = &app.signatures.sig_1;
            let resp = TestAction::new(app).register(user_sig);
            assert_eq!(resp.status, RegistrationStatus::Registered);
        });
    });
}

#[rocket::async_test]
async fn create_cluster() {
    TestRunner::default().run(|ctx| {
        let docker_cli: Cli = clients::Cli::default();
        let container: Container<Mongo> = docker_cli.run(Mongo::default());

        let infra = MetaSecretDocker::run(&ctx, &docker_cli, &container);
        let infra = task::block_on(infra);
        println!("{:?}", infra.mongo_db_url);

        let test_app = MetaSecretTestApp::new(infra);

        test_app.actions(|app| {
            let resp = TestAction::new(app).register(&app.signatures.sig_1);
            assert_eq!(resp.status, RegistrationStatus::Registered);

            let resp = TestAction::new(app).register(&app.signatures.sig_2);
            assert_eq!(resp.status, RegistrationStatus::AlreadyExists);

            let resp = TestAction::new(app).register(&app.signatures.sig_3);
            assert_eq!(resp.status, RegistrationStatus::AlreadyExists);

            let accept_resp = TestAction::new(app).accept(&JoinRequest {
                member: app.signatures.sig_1.clone(),
                candidate: app.signatures.sig_2.clone(),
            });
            assert_eq!(accept_resp.status, MembershipStatus::Finished);

            let accept_resp = TestAction::new(app).accept(&JoinRequest {
                member: app.signatures.sig_2.clone(),
                candidate: app.signatures.sig_3.clone(),
            });
            assert_eq!(accept_resp.status, MembershipStatus::Finished);
        });
    });
}
