use crate::error::CobblepotError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, path};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct VaultConfig {
    /// location of the vault store
    vault_store: String,
}

impl Default for VaultConfig {
    fn default() -> Self {
        let vault_store = directories::UserDirs::new()
            .expect("Unable to get user-facing standard directories")
            .document_dir()
            .expect("Unable to get user-facing document directory")
            .join("/cobblepot_vault_store")
            .to_str()
            .expect("Unable to convert path to string")
            .to_string();

        VaultConfig { vault_store }
    }
}

impl VaultConfig {
    pub fn new(vault_store: String) -> Self {
        VaultConfig { vault_store }
    }

    /// Location of the vault store
    pub fn vault_store_as_pathbuf(&self) -> PathBuf {
        let vault_store = path::Path::new(&self.vault_store).to_path_buf();
        if !vault_store.exists() {
            fs::create_dir_all(vault_store.clone()).unwrap();
        }
        vault_store
    }
}

pub fn read_vault_config() -> Result<VaultConfig, CobblepotError> {
    let projects = directories::ProjectDirs::from("", "Cobblepot", "cobblepot").ok_or_else(|| {
        CobblepotError::VaultCreationError("Unable to locate systems Project directories")
    });
    match projects {
        Ok(projects) => {
            let config_path = projects.config_dir().join("cobblepot.toml");
            let config = fs::read_to_string(config_path)
                .map_err(|_| CobblepotError::VaultCreationError("Error reading vault config"))?;
            let config: VaultConfig = toml::from_str(&config)
                .map_err(|_| CobblepotError::VaultCreationError("Error parsing vault config"))?;
            Ok(config)
        },
        Err(e) => Err(e),
    }
}
