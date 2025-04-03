use diesel::{Connection, sqlite::SqliteConnection};

pub mod account;
pub mod balance;
pub mod balance_sheet;
pub mod deep_dive;
pub mod shared;

pub fn establish_connection(database_url: String) -> SqliteConnection {
    let connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Failed to connect, error: {}", e));
    connection
}
