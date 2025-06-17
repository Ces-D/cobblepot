use crate::shared::{CobblepotError, CobblepotResult};
use diesel::{
    Connection, SqliteConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn establish_connection() -> CobblepotResult<SqliteConnection> {
    let database_url = super::env::database_url()?;
    let mut connection = SqliteConnection::establish(database_url.as_str())
        .map_err(|e| CobblepotError::LogicError(e.to_string()))?;
    connection.run_pending_migrations(MIGRATIONS).expect("FAILED to run migrations");
    Ok(connection)
}

/// A type alias for a connection pool to the Sqlite database
pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

/// A type alias for a connection to the Sqlite database called from a connectin pool
pub type PoolConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

/// Creates a connection pool to the Sqlite database
pub fn database_pool() -> CobblepotResult<DbPool> {
    let database_url = super::env::database_url()?;

    // Create a connection pool to the Sqlite database
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager).map_err(|e| CobblepotError::LogicError(e.to_string()))
}
