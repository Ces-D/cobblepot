use std::{fs, path::PathBuf};

pub struct VaultConfig {
    pub vault_path: PathBuf,
}

impl VaultConfig {
    pub fn new(vault_path: PathBuf) -> VaultConfig {
        let vault_config = VaultConfig { vault_path };
        vault_config.ensure_vault_created();
        vault_config
    }

    /// Checks for the existence of three storage directories: `vault`, `chart_of_account`, and `journal`.
    /// If any of these directories does not exist, the method creates it.
    /// If there is an error during directory creation, the method panics with a descriptive error message.
    fn ensure_vault_created(&self) {
        if !&self.vault_path.exists() {
            fs::create_dir_all(&self.vault_path).expect("Error creating vault directory");
        }

        let chart_of_account_path = &self.vault_path.join("chart_of_account");
        if !chart_of_account_path.exists() {
            fs::create_dir_all(chart_of_account_path)
                .expect("Error creating chart of account directory");
        }

        let journal_path = &self.vault_path.join("journal");
        if !journal_path.exists() {
            fs::create_dir_all(journal_path).expect("Error creating journal directory");
        }
    }
}
