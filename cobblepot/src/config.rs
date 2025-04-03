#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    /// The path to the SQLite database file.
    pub connection_url: String,
}

impl Default for Config {
    fn default() -> Self {
        let data_dir = dirs::data_local_dir().expect("Failed to get data local directory");
        let connection_url = data_dir.join("cobblepot.db");
        Config {
            connection_url: connection_url
                .to_str()
                .expect("Invalid Unicode in connection url")
                .to_owned(),
        }
    }
}

impl Config {
    pub fn read() -> Config {
        let config_path = dirs::config_dir()
            .expect("Failed to get config directory")
            .join("cobblepot.json");
        if config_path.exists() {
            let file = std::fs::File::open(config_path).expect("Failed to open config file");
            let reader = std::io::BufReader::new(file);
            serde_json::de::from_reader(reader).expect("Failed to parse config file")
        } else {
            Config::default()
        }
    }
}
