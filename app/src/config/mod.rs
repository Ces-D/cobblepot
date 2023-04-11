use std::{fs, path::PathBuf};

use directories::ProjectDirs;
use serde::Deserialize;

mod vault;

#[derive(Deserialize, Debug, Clone)]
enum Currency {
    USD,
}

#[derive(Deserialize, Debug)]
pub struct CobblepotConfig {
    /// The path to the accounting store.
    vault_path: PathBuf,
    ///  The default currency used for all transactions.
    currency: Option<Currency>,
    // The format in which dates are displayed and entered.
    date_format: Option<String>,
}

impl CobblepotConfig {
    /// Create a new CobblepotConfig with default values.
    fn create_base_config() -> CobblepotConfig {
        CobblepotConfig {
            vault_path: PathBuf::from("./tmp/cobblepot"),
            currency: Some(Currency::USD),
            date_format: Some(String::from("%Y-%m-%d %H:%M:%S")),
        }
    }

    /// Load the config file from the user's config directory. If testing, change the config file
    /// path so that real cobblepot config file values are not used.
    pub fn load_config_file() -> CobblepotConfig {
        let base_config = CobblepotConfig::create_base_config();

        // Linux:   /home/alice/.config/barapp
        // Windows: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App
        // macOS:   /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
        match ProjectDirs::from("dev", "accounting", "cobblepot") {
            Some(proj_dir) => {
                println!("Project dir: {:?}", proj_dir);
                let project_config_dir = proj_dir.config_dir();
                let defined_config_contents =
                    fs::read_to_string(project_config_dir.join("cobblepot.toml"))
                        .unwrap_or("".to_string());

                let config: CobblepotConfig =
                    toml::from_str(&defined_config_contents).unwrap_or(base_config);
                config
            },
            None => base_config,
        }
    }

    // TODO: This could be a struct that has functions that convert various types into the Currency
    // formats that we want
    pub fn get_currency(&self) -> Currency {
        if self.currency.is_none() {
            let base_config = CobblepotConfig::create_base_config();
            return base_config.currency.unwrap();
        }
        self.currency.clone().unwrap()
    }

    // TODO: This should return more than a string, maybe it is a function that does the actual
    // formatting of a string into the appropriate date format.
    pub fn get_date_format(&self) -> String {
        if self.date_format.is_none() {
            let base_config = CobblepotConfig::create_base_config();
            return base_config.date_format.unwrap();
        }
        self.date_format.clone().unwrap()
    }
}

// TODO (Cesar): This is more important than the other todays above. The various config files in
// this module should handle reading the config file properties and transforming them into whatever
// format is more useful for the application. The vault file, should create the various sub
// directories in the vault store. The dates file should have formatting for various types of dates. The currency file should have formatting for various types of acurrency methods, including exporting the Currency enum
