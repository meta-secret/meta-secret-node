use async_std::task;
use meta_secret_core::crypto::key_pair::KeyPair;
use meta_secret_core::crypto::keys::AeadCipherText;
use meta_secret_core::shared_secret;
use meta_secret_core::shared_secret::data_block::common::SharedSecretConfig;
use meta_secret_core::shared_secret::shared_secret::UserShareDto;
use testcontainers::clients::Cli;
use testcontainers::images::mongo::Mongo;
use testcontainers::{clients, Container};

use meta_secret_vault_server_lib::api::api::{EncryptedMessage, MetaPasswordRequest};
use meta_secret_vault_server_lib::api::api::{RegistrationStatus, UserSignature};
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
        println!("mongodb url: {:?}", infra.mongo_db_url);

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

            let km_1 = &app.signatures.key_manager_1;
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

            TestAction::new(app).distribute_password(&split_request);
        });
    });
}

#[rocket::async_test]
async fn split_and_recover_password() {
    TestRunner::default().run(|ctx| {
        let docker_cli: Cli = clients::Cli::default();
        let container: Container<Mongo> = docker_cli.run(Mongo::default());

        let infra = MetaSecretDocker::run(&ctx, &docker_cli, &container);
        let infra = task::block_on(infra);

        let test_app = MetaSecretTestApp::new(infra);

        test_app.actions(|app| {
            TestAction::new(app).create_cluster();

            let user_sig: &UserSignature = &app.signatures.sig_1;
            let vault = TestAction::new(app).get_vault(user_sig);
            println!(
                "vault info: {}",
                serde_json::to_string_pretty(&vault).unwrap()
            );

            //split password to 3 shares and send 3 requests to distribute the password
            let cfg = SharedSecretConfig {
                number_of_shares: 3,
                threshold: 2,
            };
            let shares = shared_secret::split("top$ecret".to_string(), cfg);
            assert_eq!(shares.len(), 3);
            println!(
                "Password shares: {}",
                serde_json::to_string_pretty(&shares).unwrap()
            );

            let sender_key_manager = app.signatures.all_key_managers()[0];

            for i in 1..shares.len() {
                let password_share: &UserShareDto = &shares[i];
                let receiver_key_manager = app.signatures.all_key_managers()[i];
                let all_sigs = app.signatures.all_signatures();

                let encrypted_share: AeadCipherText =
                    sender_key_manager.transport_key_pair.encrypt_string(
                        serde_json::to_string(&password_share).unwrap(),
                        receiver_key_manager.transport_key_pair.public_key(),
                    );

                let split_request = SecretDistributionDoc {
                    distribution_type: SecretDistributionType::Split,
                    meta_password: MetaPasswordRequest {
                        user_sig: user_sig.clone(),
                        meta_password: MetaPasswordDoc {
                            id: MetaPasswordId::new("test".to_string(), "salt-salt".to_string()),
                            vault: vault.vault.clone().unwrap(),
                        },
                    },
                    secret_message: EncryptedMessage {
                        receiver: all_sigs[i].clone(),
                        encrypted_text: encrypted_share,
                    },
                };

                TestAction::new(app).distribute_password(&split_request);
            }

            // 1. All devices need to get shares
            // 2. all devices save shares locally
            // 3. device_1 sends distribute request with "recover" type
            // 4. other devices read distribute request and send their shares of the password to the server
            // 5. device_1 reads shares and restores password

            //restore
            let device_1_shares = TestAction::new(app).find_shares(user_sig);
            println!(
                "shares to distribute for device_1: {}",
                serde_json::to_string_pretty(&device_1_shares).unwrap()
            );
        });
    });
}
