use diesel::{Connection, sqlite::SqliteConnection};
use std::env;

pub mod account;
pub mod balance;
pub mod balance_sheet;
pub mod deep_dive;
pub mod shared;

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Failed to connect, error: {}", e));
    connection
}
