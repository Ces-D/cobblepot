use rusty_money::iso::{find_by_num_code, Currency};

mod config_file;
mod dates;
mod vault;

/// Public unit of work for the applications config.
pub struct Config {
    pub vault_config: vault::VaultConfig,
    pub dates_config: dates::DateConfig,
    pub currency_config: Currency,
}

impl Config {
    pub fn new() -> Config {
        let config_file = config_file::load_config_file();

        Config {
            vault_config: vault::VaultConfig::new(config_file.vault_path),
            dates_config: dates::DateConfig::new(config_file.date_format),
            currency_config: find_by_num_code(&config_file.currency.unwrap_or("USD".to_string()))
                .unwrap_or(rusty_money::iso::USD)
                .clone(),
        }
    }
}

// TODO: implement the config file in the application, it is not referenced or used anywhere
