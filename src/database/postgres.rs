use std::error::Error;
use std::time::Duration;

use crate::config::Config;
use actix_web::web::Data;
use diesel::Connection;
use diesel_async::AsyncPgConnection;
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, PoolError};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

pub type PgPool = Pool<AsyncPgConnection>;
pub type PgPooled<'a> = PooledConnection<'a, AsyncPgConnection>;

pub async fn pool(config: &Data<Config>) -> Result<PgPool, PoolError> {
    let mut pool = Pool::builder();
    if let Some(pool_connection_timeout) = config.pool_connection_timeout {
        pool = pool.connection_timeout(Duration::from_secs(pool_connection_timeout));
    }
    if let Some(pool_max_connections) = config.pool_max_connections {
        pool = pool.max_size(pool_max_connections);
    }
    if config.pool_min_reserved_connections.is_some() {
        pool = pool.min_idle(config.pool_min_reserved_connections);
    }

    pool.build(AsyncDieselConnectionManager::<AsyncPgConnection>::new(
        &config.database_url,
    ))
    .await
}

pub async fn run_migrations(
    database_url: String,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    actix_web::web::block(move || {
        AsyncConnectionWrapper::<AsyncPgConnection>::establish(&database_url)?
            .run_pending_migrations(MIGRATIONS)?;

        Ok(())
    })
    .await?
}
