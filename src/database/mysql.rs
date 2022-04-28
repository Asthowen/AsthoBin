use diesel::r2d2::{self, ConnectionManager, Pool};
use crate::util::utils::get_key;
use diesel::prelude::*;

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn get_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    let manager: ConnectionManager<MysqlConnection> = ConnectionManager::<MysqlConnection>::new(
        get_key("DATABASE_URL")
    );
    r2d2::Pool::builder()
        .connection_timeout(std::time::Duration::from_secs(1))
        .build(manager)
        .map_err(|err| {
            log::error!("{}", err.to_string());
            std::process::exit(9);
        })
        .unwrap()
}