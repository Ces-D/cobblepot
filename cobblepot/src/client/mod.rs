use diesel::{Connection, sqlite::SqliteConnection};

pub mod account;
pub mod balance;
pub mod balance_sheet;
pub mod deep_dive;
pub mod shared;

pub fn establish_connection() -> SqliteConnection {
    let database_url = std::env::var("COBBLEPOT_DATABASE_URL")
        .expect("COBBLEPOT_DATABASE_URL must be set in environment");
    let connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Failed to connect, error: {}", e));
    connection
}
