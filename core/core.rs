use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::OnceLock};

const COBBLEPOT_CONFIG_DIR: &str = "COBBLEPOT_CONFIG_DIR";
const COBBLEPOT_LOCAL_DATA_PATH: &str = ".local/share/cobblepot";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    db_name: String,
    tiingo_api_key: Option<String>,
    #[serde(skip_deserializing, skip_serializing)]
    is_production: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            db_name: "cobblepot.data".to_string(),
            tiingo_api_key: Default::default(),
            is_production: Default::default(),
        }
    }
}
impl Config {
    /// Initializes the global config
    pub fn init(is_production: bool) {
        log::trace!("Is production = {}", is_production);
        let config_path = config_location();
        let location = config_path.join("config.toml");
        if location.exists() && location.is_file() {
            let content = std::fs::read_to_string(location).expect("Failed to read config file");
            let mut config = toml::from_str::<Self>(&content).expect("Invalid toml in config file");
            config.is_production = is_production;
            CONFIG.set(config).expect("Failed to load cobblepot config");
        } else {
            let default_config = toml::to_string_pretty(&Config::default())
                .expect("Failed to serialize default config");
            std::fs::write(&location, default_config).expect("Failed to write default config file");
            log::info!("Wrote config file at {}", location.display());
            let mut config = Config::default();
            config.is_production = is_production;
            CONFIG.set(config).expect("Failed to initialize config");
        }
    }
    /// After initialization, call config through this global
    pub fn global() -> &'static Config {
        CONFIG.get().expect("Config not initialized. Call Config::init() first.")
    }
    pub fn database_url(&self) -> String {
        let database_name = match &self.is_production {
            true => self.db_name.clone(),
            false => "test_data_can_delete.db".to_string(),
        };
        let path = std::env::home_dir().expect("Unable to locate home directory");
        let storage_dir = path.join(COBBLEPOT_LOCAL_DATA_PATH);
        if !storage_dir.exists() {
            log::info!("Creating application storage directory at {}", storage_dir.display());
            std::fs::create_dir_all(&storage_dir).expect("Failed to create data local directory");
        }
        let db_path = storage_dir.join(database_name);
        log::trace!("Database url: {}", db_path.display());
        format!("{}", db_path.display())
    }
    /// The Tiingo fincial markets api key.
    pub fn financial_api_key(&self) -> String {
        if self.tiingo_api_key.is_none() {
            panic!("Missing `tiingo_api_key` in config");
        } else {
            self.tiingo_api_key.clone().unwrap()
        }
    }
}

static CONFIG: OnceLock<Config> = OnceLock::new();

fn config_location() -> PathBuf {
    match std::env::var(COBBLEPOT_CONFIG_DIR) {
        Ok(location) => {
            let mut path = PathBuf::new();
            path.push(location);
            path
        }
        Err(e) => match e {
            std::env::VarError::NotPresent => {
                let path = std::env::home_dir().expect("Unable to locate home directory");
                let config_dir = path.join(".config").join("cobblepot");
                if !config_dir.exists() {
                    log::info!("Creating config directory: {}", config_dir.display());
                    std::fs::create_dir_all(config_dir).expect("Creating config directory");
                }
                path.join(".config").join("cobblepot")
            }
            std::env::VarError::NotUnicode(os_string) => {
                panic!("Failed to interpret {} env var: {:?}", COBBLEPOT_CONFIG_DIR, os_string)
            }
        },
    }
}
