use mongodb::bson::doc;

use meta_secret_vault_server_lib::api::api::UserSignature;
use meta_secret_vault_server_lib::db::{Db, VaultDoc};
use meta_secret_vault_server_lib::restful_api::commons;

pub struct UserSignatureSpec {
    pub user_sig: UserSignature,
}

impl UserSignatureSpec {
    pub fn check(&self) {
        //check signature!
        self.user_sig.validate().unwrap()
    }
}

pub struct VaultDocDesiredState {
    pub signatures_num: usize,
    pub declined_joins_num: usize,
    pub pending_joins_num: usize,
}

impl Default for VaultDocDesiredState {
    fn default() -> Self {
        Self {
            signatures_num: 1,
            declined_joins_num: 0,
            pending_joins_num: 0,
        }
    }
}

pub struct VaultDocSpec {
    pub vault: VaultDoc,
    pub expected: VaultDocDesiredState,
}

impl VaultDocSpec {
    pub fn check(&self) {
        let vault = &self.vault;
        if vault.signatures.len() != self.expected.signatures_num {
            panic!("Invalid vault. Must be only one signature");
        }

        if vault.declined_joins.len() != self.expected.declined_joins_num {
            panic!("Wrong number of declined joins");
        }

        if vault.pending_joins.len() != self.expected.pending_joins_num {
            panic!("Wrong number of pending joins");
        }
    }
}

pub struct RegisterSpec<'a> {
    pub db: &'a Db,
    pub user_sig_spec: UserSignatureSpec,
}

impl<'a> RegisterSpec<'a> {
    pub async fn check(&self) {
        self.user_sig_spec.check();

        let vault_name = &self.user_sig_spec.user_sig.vault_name;
        let vaults_col = self.db.vaults_col();
        let filter = doc! {
            "vaultName": vault_name,
        };
        let vaults_num = vaults_col.count_documents(filter, None).await.unwrap();

        if vaults_num != 1 {
            panic!(
                "There must be only one vault with name: {}, but there are: {} vaults",
                vault_name, vaults_num
            );
        }

        let vault = commons::find_vault(self.db, &self.user_sig_spec.user_sig)
            .await
            .unwrap();

        let vault_spec = VaultDocSpec {
            vault,
            expected: Default::default(),
        };
        vault_spec.check();
    }
}
