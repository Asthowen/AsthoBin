use crate::util::utils::{get_key, WAIT_TWO_SECONDS};
use bb8::{Pool, PooledConnection};
use diesel::prelude::*;
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncMysqlConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub type MysqlPool = Pool<AsyncDieselConnectionManager<AsyncMysqlConnection>>;
pub type MysqlPooled<'a> = PooledConnection<'a, AsyncDieselConnectionManager<AsyncMysqlConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub async fn get_pool() -> MysqlPool {
    let manager =
        AsyncDieselConnectionManager::<AsyncMysqlConnection>::new(get_key("DATABASE_URL"));

    Pool::builder()
        .connection_timeout(WAIT_TWO_SECONDS)
        .build(manager)
        .await
        .unwrap_or_else(|e| {
            log::error!("{}", e.to_string());
            std::process::exit(9);
        })
}

pub async fn run_migration() {
    tokio::task::spawn_blocking(move || {
        let mut conn =
            AsyncConnectionWrapper::<AsyncMysqlConnection>::establish(&get_key("DATABASE_URL"))
                .unwrap_or_else(|e| {
                    log::error!(
                        "Error when connecting to database to deploy migrations: {}",
                        e.to_string()
                    );
                    std::process::exit(1);
                });
        conn.run_pending_migrations(MIGRATIONS).unwrap_or_else(|e| {
            log::error!("Error when deploying migrations: {}", e.to_string());
            std::process::exit(1);
        });
    })
    .await
    .unwrap_or_else(|e| {
        log::error!("Error when deploying migrations: {}", e.to_string());
        std::process::exit(1);
    });
}
