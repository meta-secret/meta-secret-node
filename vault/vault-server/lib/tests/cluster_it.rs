use async_std::task;
use testcontainers::clients::Cli;
use testcontainers::images::mongo::Mongo;
use testcontainers::{clients, Container};
use tracing::info;

use crate::testing::framework::{MetaSecretTestApp, TestAction};
use crate::testing::test_infra::{MetaSecretDocker, MetaSecretDockerInfra};
use crate::testing::testify::TestRunner;

mod testing;

#[rocket::async_test]
async fn create_cluster() {
    MetaSecretDocker::init_logging();

    let test_runner = TestRunner::default();

    let docker_cli: Cli = clients::Cli::default();
    let container: Container<Mongo> = docker_cli.run(Mongo::default());

    let infra = MetaSecretDocker::run(&test_runner.fixture, &docker_cli, &container);
    let infra = task::block_on(infra);
    info!("{:?}", infra.mongo_db_url);

    let test_app = MetaSecretTestApp::new(infra);

    test_app.actions(|app| {
        TestAction::new(app).create_cluster();
    });
}
