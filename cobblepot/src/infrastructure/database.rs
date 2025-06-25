use crate::shared::{CobblepotError, CobblepotResult};
use diesel::{
    SqliteConnection,
    r2d2::ManageConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// A type alias for a connection pool to the Sqlite database
pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

/// A type alias for a connection to the Sqlite database called from a connectin pool
pub type PoolConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

/// Creates a connection pool to the Sqlite database
pub fn database_pool() -> CobblepotResult<DbPool> {
    let database_url = super::env::database_url()?;
    // Create a connection pool to the Sqlite database
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let mut migration_conn = manager.connect().unwrap();
    let migrations = migration_conn.run_pending_migrations(MIGRATIONS).unwrap();
    println!("Ran migrations - {}", migrations.len());
    Pool::builder().build(manager).map_err(|e| CobblepotError::LogicError(e.to_string()))
}

#[cfg(test)]
/// Creates a connection pool to the Sqlite in-memory database. Runs migrations on each connection
pub fn database_memory_pool() -> CobblepotResult<DbPool> {
    use diesel::r2d2::CustomizeConnection;

    #[derive(Debug)]
    struct TestConnectionCustomizer;

    impl CustomizeConnection<SqliteConnection, diesel::r2d2::Error> for TestConnectionCustomizer {
        fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
            use diesel_migrations::MigrationHarness;
            conn.run_pending_migrations(MIGRATIONS).unwrap();
            Ok(())
        }
    }

    let database_url = ":memory:".to_string();
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .connection_customizer(Box::new(TestConnectionCustomizer))
        .build(manager)
        .map_err(|e| CobblepotError::LogicError(e.to_string()))
}
