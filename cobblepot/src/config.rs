use diesel::{Connection, sqlite::SqliteConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use std::path::{Path, PathBuf};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// The path to the config file for Cobblepot.
fn config_path() -> PathBuf {
    let home = std::env::var("HOME").expect("Failed to get home directory");
    let config_dir = Path::new(&home).join(".config/cobblepot");
    if !config_dir.exists() {
        println!("Creating config directory at {}", config_dir.display());
        std::fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    }
    config_dir.join("cobblepot.json")
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    /// The path to the SQLite database file.
    pub connection_url: String,
}

impl Default for Config {
    fn default() -> Self {
        let home_dir = std::env::var("HOME").expect("Failed to get home directory");
        let data_dir = Path::new(&home_dir).join(".local/share/cobblepot");
        if !data_dir.exists() {
            println!("Creating data local directory at {}", data_dir.display());
            std::fs::create_dir_all(&data_dir).expect("Failed to create data local directory");
        }
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
    pub fn setup() -> Config {
        let config = Config::read();
        if config.database_exists() {
            config
        } else {
            println!("Creating database at {}", config.connection_url);
            config.create_database_with_migrations();
            println!("Database migrations successful.");
            config
        }
    }

    /// Either reads or creates a default config file
    fn read() -> Config {
        let config_path = config_path();
        if config_path.exists() {
            let file = std::fs::File::open(config_path).expect("Failed to open config file");
            let reader = std::io::BufReader::new(file);
            serde_json::de::from_reader(reader).expect("Failed to parse config file")
        } else {
            let buffer = std::fs::File::create(config_path).expect("Unable to create config file");
            let config = Config::default();
            serde_json::to_writer_pretty(buffer, &config).expect("Failed to write to config file");
            config
        }
    }

    fn database_exists(&self) -> bool {
        let path = Path::new(&self.connection_url);
        path.exists()
    }

    pub fn establish_connection(&self) -> SqliteConnection {
        let connection = SqliteConnection::establish(&self.connection_url)
            .unwrap_or_else(|e| panic!("Failed to connect, error: {}", e));
        connection
    }

    fn create_database_with_migrations(&self) {
        let mut connection = self.establish_connection();
        connection
            .run_pending_migrations(MIGRATIONS)
            .expect("Failed to run migrations");
    }
}
