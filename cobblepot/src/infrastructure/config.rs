use std::path::{Path, PathBuf};

use diesel::{Connection, SqliteConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use serde::{Deserialize, Serialize};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// The path to the SQLite database file.
    pub connection_url: PathBuf,
}

impl Config {
    pub fn open() -> Config {
        let in_production = std::env::var("RUST_ENV").unwrap_or("production".to_string());
        if in_production == "production" {
            let storage_dir = Config::storage_dir_path();
            let connection_url = storage_dir.join("cobblepot.db");
            Config {
                connection_url,
            }
        } else {
            Config {
                connection_url: std::env::current_dir()
                    .expect("Failed to get current directory")
                    .join("cobblepot.db"),
            }
        }
    }

    pub fn establish_connection(&self) -> SqliteConnection {
        let mut connection = SqliteConnection::establish(self.connection_url.to_str().unwrap())
            .unwrap_or_else(|e| panic!("Failed to establish db connection: {}", e));
        connection.run_pending_migrations(MIGRATIONS).expect("FAILED to run migrations");
        connection
    }

    fn storage_dir_path() -> PathBuf {
        let home_dir = std::env::var("HOME").expect("Failed to get home directory");
        let storage_dir = Path::new(&home_dir).join(".local/share/cobblepot");
        if !storage_dir.exists() {
            println!("Creating data local directory at {}", storage_dir.display());
            std::fs::create_dir_all(&storage_dir).expect("Failed to create data local directory");
        }
        storage_dir
    }
}
