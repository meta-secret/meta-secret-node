use async_std::task;
use testcontainers::clients::Cli;
use testcontainers::images::mongo::Mongo;
use testcontainers::{clients, Container};

use meta_secret_vault_server_lib::api::api::RegistrationStatus;
use meta_secret_vault_server_lib::api::api::{EncryptedMessage, MetaPasswordRequest};
use meta_secret_vault_server_lib::db::{
    MetaPasswordDoc, MetaPasswordId, SecretDistributionDoc, SecretDistributionType,
};

use crate::testing::framework::{MetaSecretTestApp, TestAction};
use crate::testing::test_infra::{MetaSecretDocker, MetaSecretDockerInfra};
use crate::testing::testify::TestRunner;

mod testing;

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
            TestAction::new(app).create_cluster();
        });
    });
}

#[rocket::async_test]
async fn split_password() {
    TestRunner::default().run(|ctx| {
        let docker_cli: Cli = clients::Cli::default();
        let container: Container<Mongo> = docker_cli.run(Mongo::default());

        let infra = MetaSecretDocker::run(&ctx, &docker_cli, &container);
        let infra = task::block_on(infra);

        let test_app = MetaSecretTestApp::new(infra);

        test_app.actions(|app| {
            let user_sig = app.signatures.sig_1.clone();

            TestAction::new(app).create_cluster();

            let vault = TestAction::new(app).get_vault(&user_sig);

            let split_request = SecretDistributionDoc {
                distribution_type: SecretDistributionType::Split,
                meta_password: MetaPasswordRequest {
                    user_sig: user_sig.clone(),
                    meta_password: MetaPasswordDoc {
                        id: MetaPasswordId::new("".to_string(), "".to_string()),
                        vault: vault.vault.unwrap(),
                    },
                },
                secret_message: EncryptedMessage {
                    receiver: user_sig,
                    encrypted_text: "123".to_string(),
                },
            };

            TestAction::new(app).split_password(&split_request);
        });
    });
}
