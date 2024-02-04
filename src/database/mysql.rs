use crate::util::utils::{get_key, WAIT_TWO_SECONDS};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use r2d2::PooledConnection;

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MysqlPooled = PooledConnection<ConnectionManager<MysqlConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn get_pool() -> MysqlPool {
    let manager: ConnectionManager<MysqlConnection> =
        ConnectionManager::<MysqlConnection>::new(get_key("DATABASE_URL"));
    Pool::builder()
        .connection_timeout(WAIT_TWO_SECONDS)
        .build(manager)
        .map_err(|err| {
            log::error!("{}", err.to_string());
            std::process::exit(9);
        })
        .unwrap()
}

pub fn run_migration(conn: &mut MysqlConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap_or_else(|e| {
        log::error!("Error when deploying migrations: {}", e.to_string());
        std::process::exit(1);
    });
}
