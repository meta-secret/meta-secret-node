pub mod test_framework;
pub mod test_infra;
pub mod test_spec;

pub mod testify {
    use meta_secret_vault_server_lib::db::DbSchema;

    #[derive(Clone, Debug, Default)]
    pub struct TestFixture {
        pub db_schema: DbSchema,
    }

    #[derive(Default)]
    pub struct TestRunner {
        pub fixture: TestFixture,
    }
}
