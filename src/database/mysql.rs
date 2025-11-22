use crate::utils::WAIT_TWO_SECONDS;
use diesel::prelude::*;
use diesel_async::AsyncMysqlConnection;
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

pub type MysqlPool = Pool<AsyncMysqlConnection>;
pub type MysqlPooled<'a> = PooledConnection<'a, AsyncDieselConnectionManager<AsyncMysqlConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub async fn get_pool(database_url: &str) -> MysqlPool {
    let manager = AsyncDieselConnectionManager::<AsyncMysqlConnection>::new(database_url);

    Pool::builder()
        .connection_timeout(WAIT_TWO_SECONDS)
        .build(manager)
        .await
        .unwrap_or_else(|error| {
            log::error!("Error when building the MySQL pool: {error}");
            std::process::exit(9);
        })
}

pub async fn run_migration(database_url: String) {
    tokio::task::spawn_blocking(move || {
        let mut conn = AsyncConnectionWrapper::<AsyncMysqlConnection>::establish(&database_url)
            .unwrap_or_else(|error| {
                log::error!("Error when connecting to database to deploy migrations: {error}");
                std::process::exit(1);
            });
        conn.run_pending_migrations(MIGRATIONS)
            .unwrap_or_else(|error| {
                log::error!("Error when deploying migrations: {error}");
                std::process::exit(1);
            });
    })
    .await
    .unwrap_or_else(|error| {
        log::error!("Error when deploying migrations: {error}");
        std::process::exit(1);
    });
}
