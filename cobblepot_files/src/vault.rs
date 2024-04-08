use cobblepot_core::error::CobblepotError;
use std::path::PathBuf;
use std::{fs, path};

#[derive(Debug, Clone)]
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
        path::Path::new(&self.location).to_path_buf()
    }
}

pub fn create_vault(config: Option<VaultConfig>) -> Result<(), CobblepotError> {
    let path = config.unwrap_or_default().location_as_pathbuf();
    fs::create_dir_all(path)
        .map_err(|_| CobblepotError::VaultCreationError("Error creating vault dir"))?;
    Ok(())
}
