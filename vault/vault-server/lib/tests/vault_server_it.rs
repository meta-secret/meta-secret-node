use async_std::task;
use meta_secret_core::crypto::key_pair::KeyPair;
use meta_secret_core::crypto::keys::{AeadCipherText, AeadPlainText};
use meta_secret_core::shared_secret::data_block::common::SharedSecretConfig;
use meta_secret_core::shared_secret::shared_secret::UserShareDto;
use meta_secret_core::{recover_from_shares, shared_secret};
use testcontainers::clients::Cli;
use testcontainers::images::mongo::Mongo;
use testcontainers::{clients, Container};

use meta_secret_vault_server_lib::api::api::{
    EncryptedMessage, MessageStatus, MetaPasswordRequest, PasswordRecoveryRequest,
};
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
    let ctx = TestRunner::default();

    let docker_cli: Cli = clients::Cli::default();
    let container: Container<Mongo> = docker_cli.run(Mongo::default());

    let infra = MetaSecretDocker::run(&ctx.fixture, &docker_cli, &container);
    let infra = task::block_on(infra);

    let test_app = MetaSecretTestApp::new(infra);

    let test_action = &TestAction::new(&test_app);
    test_action.create_cluster();

    let user_sig: &UserSignature = &test_app.signatures.sig_1;

    //split password to 3 shares and send 3 requests to distribute the password
    let cfg = SharedSecretConfig {
        number_of_shares: 3,
        threshold: 2,
    };

    let sender_key_manager = test_app.signatures.all_key_managers()[0];

    let meta_password_id = MetaPasswordId::new("test".to_string(), "salt-salt".to_string());

    let vault = test_action.get_vault(user_sig);

    let top_secret_password = "t0p$3cr3t".to_string();
    let shares: Vec<UserShareDto> = shared_secret::split(top_secret_password.clone(), cfg);
    assert_eq!(shares.len(), 3);

    let meta_password = MetaPasswordRequest {
        user_sig: user_sig.clone(),
        meta_password: MetaPasswordDoc {
            id: meta_password_id.clone(),
            vault: vault.vault.clone().unwrap(),
        },
    };

    for i in 1..shares.len() {
        let password_share: &UserShareDto = &shares[i];
        let receiver_key_manager = test_app.signatures.all_key_managers()[i];
        let all_sigs = test_app.signatures.all_signatures();

        let encrypted_share: AeadCipherText = sender_key_manager.transport_key_pair.encrypt_string(
            serde_json::to_string(&password_share).unwrap(),
            receiver_key_manager.transport_key_pair.public_key(),
        );

        let split_request = SecretDistributionDoc {
            distribution_type: SecretDistributionType::Split,
            meta_password: meta_password.clone(),
            secret_message: EncryptedMessage {
                receiver: all_sigs[i].clone(),
                encrypted_text: encrypted_share,
            },
        };

        test_action.distribute_password(&split_request);
    }

    // - all devices need to get shares
    // - all devices save shares locally

    //restore
    let device_2_shares = test_action.find_shares(&test_app.signatures.sig_2);
    println!(
        "shares to distribute for device_2: {}",
        serde_json::to_string_pretty(&device_2_shares).unwrap()
    );

    let device_3_shares = test_action.find_shares(&test_app.signatures.sig_3);
    println!(
        "shares to distribute for device_3: {}",
        serde_json::to_string_pretty(&device_3_shares).unwrap()
    );

    //get all meta_passwords???

    // - device_1 sends "claim_for_password_recovery" request with "recover" type
    let pass_recovery_request_device_2 = PasswordRecoveryRequest {
        id: meta_password_id.clone(),
        consumer: user_sig.clone(),
        provider: test_app.signatures.sig_2.clone(),
    };

    println!("Claim for password recovery for device_2");
    let recovery_request_device_2 = test_action.claim_for_password_recovery(&pass_recovery_request_device_2);
    assert_eq!(recovery_request_device_2, MessageStatus::Ok);

    let pass_recovery_request_device_3 = PasswordRecoveryRequest {
        id: meta_password_id,
        consumer: user_sig.clone(),
        provider: test_app.signatures.sig_3.clone(),
    };
    println!("Claim for password recovery for device_3");
    let recovery_request_device_3 = test_action.claim_for_password_recovery(&pass_recovery_request_device_3);
    assert_eq!(recovery_request_device_3, MessageStatus::Ok);

    //devices read claims
    println!("Device_2: find password recovery claims");
    let sig_2_claim = test_action
        .find_password_recovery_claims(&test_app.signatures.sig_2)
        .await;
    assert_eq!(sig_2_claim.len(), 1);

    println!("Device_3: find password recovery claims");
    let sig_3_claim = test_action
        .find_password_recovery_claims(&test_app.signatures.sig_3)
        .await;
    assert_eq!(sig_3_claim.len(), 1);

    // - other devices read "claim_for_password_recovery request and send their shares of the password to the server
    // - device_1 reads shares and restores password

    //decrypt a share on the device and encrypt with a receiver device and send it to "distribute" method
    let d_2_share: &SecretDistributionDoc = &device_2_shares[0];
    let d_2_transport = &test_app.signatures.key_manager_2.transport_key_pair;
    let decrypted_d2 = d_2_transport.decrypt(&d_2_share.secret_message.encrypted_text);

    let d_2_share_encrypted_for_d_1: AeadCipherText =
        d_2_transport.encrypt_string(decrypted_d2.msg.clone(), decrypted_d2.auth_data.sender_public_key);

    let d2_recovery_request = SecretDistributionDoc {
        distribution_type: SecretDistributionType::Recover,
        meta_password: meta_password.clone(),
        secret_message: EncryptedMessage {
            receiver: test_app.signatures.sig_1.clone(),
            encrypted_text: d_2_share_encrypted_for_d_1,
        },
    };

    test_action.distribute_password(&d2_recovery_request);

    //Device_1 gets the share, decrypts, restores password
    let pass_shares_for_device_1 = test_action.find_shares(user_sig);
    let pass_share_for_device_1: &SecretDistributionDoc = &pass_shares_for_device_1[0];
    assert_eq!(
        pass_share_for_device_1.distribution_type,
        SecretDistributionType::Recover
    );

    let share_from_device_2_json: AeadPlainText = test_app
        .signatures
        .key_manager_1
        .transport_key_pair
        .decrypt(&pass_share_for_device_1.secret_message.encrypted_text);

    let share_from_device_2_json: UserShareDto = serde_json::from_str(&share_from_device_2_json.msg).unwrap();
    println!("Decrypted share from device 2 {:?}", share_from_device_2_json);

    let device_1_password_share: UserShareDto = shares[0].clone();

    let password = recover_from_shares(vec![share_from_device_2_json, device_1_password_share]);
    assert_eq!(top_secret_password, password.unwrap().as_string());
}
