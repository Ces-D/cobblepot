use cobblepot_core::error::CobblepotError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, path};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct VaultConfig {
    /// Optional absolute path to the vault store
    pub location: String,
}

impl Default for VaultConfig {
    fn default() -> Self {
        let location = directories::UserDirs::new()
            .expect("Unable to get user-facing standard directories")
            .document_dir()
            .expect("Unable to get user-facing document directory")
            .join("/cobblepot_vault_store")
            .to_str()
            .expect("Unable to convert path to string")
            .to_string();

        VaultConfig { location }
    }
}

impl VaultConfig {
    pub fn new(location: String) -> Self {
        VaultConfig { location }
    }

    pub fn location_as_pathbuf(&self) -> PathBuf {
        let p = path::Path::new(&self.location).to_path_buf();
        p
    }

    pub fn create_vault(&self) -> Result<(), CobblepotError> {
        let p = self.location_as_pathbuf();
        if !p.exists() {
            fs::create_dir_all(p.clone())
                .map_err(|_| CobblepotError::VaultCreationError("Error creating vault"))?;
        }
        Ok(())
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
