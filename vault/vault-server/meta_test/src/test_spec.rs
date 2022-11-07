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
#[derive(Debug, Clone)]
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

impl VaultDocDesiredState {
    pub fn one_member_one_pending() -> Self {
        Self {
            signatures_num: 1,
            declined_joins_num: 0,
            pending_joins_num: 1,
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

pub struct DbVaultSpec<'a> {
    pub vault_name: String,
    pub db: &'a Db,
}

impl<'a> DbVaultSpec<'a> {
    pub async fn check(&self) {
        let vaults_col = self.db.vaults_col();
        let filter = doc! {
            "vaultName": &self.vault_name,
        };
        let vaults_num = vaults_col.count_documents(filter, None).await.unwrap();

        if vaults_num != 1 {
            panic!(
                "There must be only one vault with name: {}, but there are: {} vaults",
                self.vault_name, vaults_num
            );
        }
    }
}

pub struct RegisterSpec<'a> {
    pub db: &'a Db,
    pub db_vault_spec: DbVaultSpec<'a>,
    pub user_sig_spec: UserSignatureSpec,
}

impl<'a> RegisterSpec<'a> {
    pub async fn check(&self) {
        self.user_sig_spec.check();
        self.db_vault_spec.check().await;

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

pub struct RegisterInClusterSpec<'a> {
    pub db: &'a Db,

    pub member_user_sig_spec: UserSignatureSpec,
    pub candidate_user_sig_spec: UserSignatureSpec,

    pub db_vault_spec: DbVaultSpec<'a>,
    pub expected_vault_state: VaultDocDesiredState,
}

impl<'a> RegisterInClusterSpec<'a> {
    /// Check that the second device in a pending state
    pub async fn check(&self) {
        self.member_user_sig_spec.check();
        self.candidate_user_sig_spec.check();

        self.db_vault_spec.check().await;

        let vault = commons::find_vault(self.db, &self.member_user_sig_spec.user_sig)
            .await
            .unwrap();

        let member_sig = vault.signatures[0].clone();
        let pending_sig = vault.pending_joins[0].clone();

        println!("vault: {:?}", vault);

        let vault_spec = VaultDocSpec {
            vault,
            expected: self.expected_vault_state.clone(),
        };
        vault_spec.check();

        if member_sig != self.member_user_sig_spec.user_sig {
            panic!(
                "Vault member should be: {}",
                &self.member_user_sig_spec.user_sig.public_key.base64_text
            );
        }

        if pending_sig != self.candidate_user_sig_spec.user_sig {
            panic!(
                "Pending member should be: {}",
                &self.candidate_user_sig_spec.user_sig.public_key.base64_text
            );
        }
    }
}
