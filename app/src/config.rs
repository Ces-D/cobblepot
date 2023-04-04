use std::fs;

use directories::ProjectDirs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
enum Currency {
    USD,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    /// The path to the accounting store.
    vault_path: String,
    ///  The default currency used for all transactions.
    currency: Currency,
    // The format in which dates are displayed and entered.
    date_format: String,
    ///  Include ability to backup and restore data in case of data loss or corruption.
    backup: bool,
}

impl Config {
    pub fn load() -> Config {
        // TODO (Cesar) - set real default dev values for this config struct
        let base_config = Config {
            vault_path: String::from(""),
            currency: Currency::USD,
            date_format: String::from(""),
            backup: false,
        };
        match ProjectDirs::from("dev", "accounting", "cobblepot") {
            Some(proj_dir) => {
                let config_dir = proj_dir.config_dir();
                let config_file = fs::read_to_string(config_dir.join("cobblepot.toml"));
                let config: Config = match config_file {
                    Ok(file) => toml::from_str(&file).unwrap(),
                    Err(_) => base_config,
                };
                config
            },
            None => base_config,
        }
    }
}
