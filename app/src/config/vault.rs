struct CobblepotVaultConfig {}

impl CobblepotVaultConfig {
    pub fn ensure_store_created(vault_path: &PathBuf) {
        if !vault_path.exists() {
            fs::create_dir_all(vault_path).expect("Error creating vault directory");
        }
    }
}
