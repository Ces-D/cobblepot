use cobblepot_files::vault::VaultConfig;

/// TODO: create tests for the configs that are passed in by the user
pub fn create_test_config() -> VaultConfig {
    let config = VaultConfig::new("../../cli_test_vault_store/".to_string());
    config.create_vault().unwrap();
    config
}
