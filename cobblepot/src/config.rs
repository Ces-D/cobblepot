use diesel::{Connection, sqlite::SqliteConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

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

    fn read() -> Config {
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

    fn database_exists(&self) -> bool {
        let path = std::path::Path::new(&self.connection_url);
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
