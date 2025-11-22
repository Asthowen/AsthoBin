use crate::api_error::ApiError;
use crate::config::Config;
use actix_web::web::Data;
use diesel::IntoSql;
use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
use diesel_async::pooled_connection::{
    AsyncDieselConnectionManager, ManagerConfig, PoolError, RecyclingMethod,
};
use diesel_async::{AsyncConnection, AsyncMigrationHarness, AsyncPgConnection, RunQueryDsl};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use std::time::Duration;

pub type PgPool = Pool<AsyncPgConnection>;
pub type PgPooled<'a> = PooledConnection<'a, AsyncPgConnection>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub async fn get_pool(config: &Data<Config>) -> Result<PgPool, PoolError> {
    // See: https://github.com/weiznich/diesel_async/issues/139
    let mut manager_config = ManagerConfig::default();
    manager_config.recycling_method = RecyclingMethod::CustomFunction(Box::new(|conn| {
        Box::pin(async move {
            let _: i32 = diesel::select(1_i32.into_sql::<diesel::sql_types::Integer>())
                .first(conn)
                .await
                .map_err(|error| {
                    log::error!("Error pinging database connection: {error}");
                    error
                })?;
            Ok(())
        })
    }));

    let manager: AsyncDieselConnectionManager<AsyncPgConnection> =
        AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(
            &config.database_url,
            manager_config,
        );

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

    pool.build(manager).await
}

pub async fn run_migration(database_url: &str) -> Result<(), ApiError> {
    let conn = AsyncPgConnection::establish(database_url)
        .await
        .unwrap_or_else(|error| {
            log::error!("Error when connecting to database to deploy migrations: {error}",);
            std::process::exit(1);
        });

    AsyncMigrationHarness::new(conn)
        .run_pending_migrations(MIGRATIONS)
        .map_err(|error| ApiError::new_log_internal(error.to_string()))?;

    Ok(())
}
