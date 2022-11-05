use async_std::task;
use meta_test::test_framework::{MetaSecretTestApp, TestAction};
use meta_test::test_infra::{MetaSecretDocker, MetaSecretDockerInfra};
use meta_test::testify::TestRunner;
use testcontainers::clients::Cli;
use testcontainers::images::mongo::Mongo;
use testcontainers::{clients, Container};
use tracing::info;

#[rocket::async_test]
async fn create_cluster() {
    MetaSecretDocker::init_logging();

    let test_runner = TestRunner::default();

    let docker_cli: Cli = clients::Cli::default();
    let container: Container<Mongo> = docker_cli.run(Mongo::default());

    let infra = MetaSecretDocker::run(&test_runner.fixture, &docker_cli, &container);
    let infra = task::block_on(infra);
    info!("{:?}", infra.mongo_db_url);

    let test_app = MetaSecretTestApp::new(&infra);
    TestAction::new(&test_app).create_cluster();
}
