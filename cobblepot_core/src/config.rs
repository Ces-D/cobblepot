use crate::error::CobblepotError;
use log::info;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, path};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct VaultConfig {
    /// location of the vault db store
    vault_database_url: String,
}

impl Default for VaultConfig {
    fn default() -> Self {
        let vault_database_url = directories::UserDirs::new()
            .expect("Unable to get user-facing standard directories")
            .document_dir()
            .expect("Unable to get user-facing document directory")
            .join("/cobblepot_vault_store")
            .to_str()
            .expect("Unable to convert path to string")
            .to_string();

        VaultConfig { vault_database_url }
    }
}

impl VaultConfig {
    /// Location of the vault store
    pub fn vault_database_url(&self) -> PathBuf {
        path::Path::new(&self.vault_database_url).to_path_buf()
    }
}

fn create_database_dir_url(database_url: PathBuf) {
    fs::create_dir_all(database_url.clone()).unwrap_or_else(|_| {
        CobblepotError::VaultCreationError("Unable to create vault database directory");
    });
    info!("Created vault_database_url: {}", database_url.to_str().unwrap());
}

pub fn read_vault_config() -> Result<VaultConfig, CobblepotError> {
    let projects = directories::ProjectDirs::from("", "Cobblepot", "cobblepot").ok_or_else(|| {
        CobblepotError::VaultCreationError("Unable to locate systems Project directories")
    });
    match projects {
        Ok(projects) => {
            let config_path = projects.config_dir().join("cobblepot.toml");
            if config_path.exists() {
                let config = fs::read_to_string(config_path).map_err(|_| {
                    CobblepotError::VaultCreationError("Error reading vault config")
                })?;
                let config: VaultConfig = toml::from_str(&config).map_err(|_| {
                    CobblepotError::VaultCreationError("Error parsing vault config")
                })?;

                if !config.vault_database_url().is_dir() {
                    create_database_dir_url(config.vault_database_url())
                }
                Ok(config)
            } else {
                let config = VaultConfig::default();
                if !config.vault_database_url().is_dir() {
                    create_database_dir_url(config.vault_database_url())
                }
                Ok(config)
            }
        },
        Err(e) => Err(e),
    }
}
