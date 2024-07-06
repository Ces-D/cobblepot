use cobblepot_core::config;
use diesel::sqlite::SqliteConnection;
use diesel::Connection;

pub mod crud;
pub mod models;
pub mod schema;

pub fn establish_connection(config: Box<config::VaultConfig>) -> SqliteConnection {
    let vault_store = config.vault_database_url();
    let vault_store = vault_store.to_str().expect("Unreadable vault store value");
    SqliteConnection::establish(vault_store)
        .unwrap_or_else(|_| panic!("Error connecting to {}", vault_store))
}
