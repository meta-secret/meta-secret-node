use async_std::task;

use rocket::http::{ContentType, Status};

use rocket::uri;
use testcontainers::clients::Cli;
use testcontainers::images::mongo::Mongo;
use testcontainers::{clients, Container};

use meta_secret_vault_server_lib::api::api::{
    RegistrationResponse, RegistrationStatus, UserSignature,
};
use meta_secret_vault_server_lib::restful_api;

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

#[rocket::async_test]
async fn sing_up_one_device() {
    TestRunner::default().run(|ctx| {
        let docker_cli: Cli = clients::Cli::default();
        let container: Container<Mongo> = docker_cli.run(Mongo::default());

        let infra = MetaSecretDocker::run(&ctx, &docker_cli, &container);
        let infra = task::block_on(infra);
        println!("{:?}", infra.mongo_db_url);

        let test_app = MetaSecretTestApp::new(infra);

        test_app.actions(|app| {
            let user_sig = &app.signatures.sig_1;
            let signup_response = app
                .infra
                .rocket_client
                .post(uri!(restful_api::register::register))
                .header(ContentType::JSON)
                .body(serde_json::to_string_pretty(&user_sig).unwrap())
                .dispatch();

            let signup_response = task::block_on(signup_response);
            assert_eq!(signup_response.status(), Status::Ok);

            let resp = signup_response.into_json::<RegistrationResponse>();
            let resp = task::block_on(resp);
            assert_eq!(resp.unwrap().status, RegistrationStatus::Registered);
        });
    });
}
