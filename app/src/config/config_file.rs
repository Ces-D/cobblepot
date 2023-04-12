use std::{fs, path::PathBuf};

use directories::ProjectDirs;
use serde::Deserialize;

use super::currency::Currency;

#[derive(Deserialize, Debug)]
pub struct CobblepotConfigFile {
    /// The path to the accounting store.
    pub vault_path: PathBuf,
    ///  The default currency used for all transactions.
    pub currency: Option<Currency>,
    // The format in which dates are displayed and entered.
    pub date_format: Option<String>,
}

impl CobblepotConfigFile {
    /// Create a new CobblepotConfig with default values.
    fn create_base_config() -> CobblepotConfigFile {
        CobblepotConfigFile {
            vault_path: PathBuf::from("./tmp/cobblepot"),
            currency: Some(Currency::USD),
            date_format: Some(String::from("%Y-%m-%d %H:%M:%S")),
        }
    }
}
/// Load the config file from the user's config directory. If testing, change the config file
/// path so that real cobblepot config file values are not used.
pub fn load_config_file() -> CobblepotConfigFile {
    let base_config = CobblepotConfigFile::create_base_config();

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

            let config: CobblepotConfigFile =
                toml::from_str(&defined_config_contents).unwrap_or(base_config);
            config
        },
        None => base_config,
    }
}
