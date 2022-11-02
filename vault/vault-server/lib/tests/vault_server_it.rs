use meta_secret_core::crypto::key_pair::KeyPair;
use testcontainers::clients::Cli;
use testcontainers::images::mongo::Mongo;
use testcontainers::{clients, Container};
use tracing::info;

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
async fn stats() {
    MetaSecretDocker::init_logging();

    let test_runner = TestRunner::default();
    let docker_cli: Cli = clients::Cli::default();
    let container: Container<Mongo> = docker_cli.run(Mongo::default());

    let infra = MetaSecretDocker::run(&test_runner.fixture, &docker_cli, &container).await;

    let test_app = MetaSecretTestApp::new(infra);
    let resp = TestAction::new(&test_app).stats();
    assert_eq!(resp.await.registrations, 0);
}

#[rocket::async_test]
async fn register_one_device() {
    MetaSecretDocker::init_logging();

    let test_runner = TestRunner::default();
    let docker_cli: Cli = clients::Cli::default();
    let container: Container<Mongo> = docker_cli.run(Mongo::default());

    let infra = MetaSecretDocker::run(&test_runner.fixture, &docker_cli, &container).await;
    info!("mongodb url: {:?}", infra.mongo_db_url);

    let test_app = MetaSecretTestApp::new(infra);

    let user_sig = &test_app.signatures.sig_1;
    let resp = TestAction::new(&test_app).register(user_sig);
    assert_eq!(resp.status, RegistrationStatus::Registered);
}

#[rocket::async_test]
async fn split_password() {
    MetaSecretDocker::init_logging();

    let test_runner = TestRunner::default();
    let docker_cli: Cli = clients::Cli::default();
    let container: Container<Mongo> = docker_cli.run(Mongo::default());

    let infra = MetaSecretDocker::run(&test_runner.fixture, &docker_cli, &container).await;

    let test_app = MetaSecretTestApp::new(infra);
    let test_action = TestAction::new(&test_app);

    let user_sig = test_app.signatures.sig_1.clone();

    test_action.create_cluster();

    let vault = test_action.get_vault(&user_sig);

    let km_1 = &test_app.signatures.key_manager_1;
    let cipher_text = km_1
        .transport_key_pair
        .encrypt_string("test".to_string(), km_1.transport_key_pair.public_key());

    let split_request = SecretDistributionDoc {
        distribution_type: SecretDistributionType::Split,
        meta_password: MetaPasswordRequest {
            user_sig: user_sig.clone(),
            meta_password: MetaPasswordDoc {
                id: MetaPasswordId::new("test".to_string(), "salt-salt".to_string()),
                vault: vault.vault.unwrap(),
            },
        },
        secret_message: EncryptedMessage {
            receiver: user_sig,
            encrypted_text: cipher_text,
        },
    };

    test_action.distribute_password(&split_request);
}
