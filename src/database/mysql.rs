use crate::util::utils::get_key;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn get_pool() -> MysqlPool {
    let manager: ConnectionManager<MysqlConnection> =
        ConnectionManager::<MysqlConnection>::new(get_key("DATABASE_URL"));
    Pool::builder()
        .connection_timeout(std::time::Duration::from_secs(1))
        .build(manager)
        .map_err(|err| {
            log::error!("{}", err.to_string());
            std::process::exit(9);
        })
        .unwrap()
}
