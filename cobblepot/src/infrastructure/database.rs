use cobblepot_core::{
    environment,
    error::{CobblepotError, CobblepotResult},
};
use diesel::{
    SqliteConnection,
    r2d2::{ConnectionManager, ManageConnection, Pool, PooledConnection},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// A type alias for a connection pool to the Sqlite database
pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

/// A type alias for a connection to the Sqlite database called from a connectin pool
pub type PoolConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

/// Creates a connection pool to the Sqlite database
pub fn database_pool() -> CobblepotResult<DbPool> {
    let database_url = environment::database_url()?;
    // Create a connection pool to the Sqlite database
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let mut migration_conn = manager.connect().unwrap();
    let migrations = migration_conn.run_pending_migrations(MIGRATIONS).unwrap();
    println!("Ran migrations - {}", migrations.len());
    Pool::builder().build(manager).map_err(|e| CobblepotError::LogicError(e.to_string()))
}

#[cfg(test)]
/// Creates a single migrated connection to an in-memory Sqlite database for testing
pub fn database_memory_pool() -> CobblepotResult<DbPool> {
    // Use a shared cache in-memory database so all connections share the same data
    let database_url = format!(
        "file:memdb{}{}?mode=memory&cache=shared",
        fastrand::alphanumeric(),
        fastrand::alphanumeric()
    );
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    // Run migrations once on a single connection before creating the pool
    let mut migration_conn =
        manager.connect().map_err(|e| CobblepotError::LogicError(e.to_string()))?;
    let migrations = migration_conn
        .run_pending_migrations(MIGRATIONS)
        .map_err(|e| CobblepotError::LogicError(e.to_string()))?;
    println!("Ran test migrations - {}", migrations.len());

    Pool::builder().build(manager).map_err(|e| CobblepotError::LogicError(e.to_string()))
}
