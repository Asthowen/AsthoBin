use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::prelude::*;
use std::env::var;

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn get_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    let url: String = var("DATABASE_URL").unwrap();
    let manager: ConnectionManager<MysqlConnection> = ConnectionManager::<MysqlConnection>::new(url);
    r2d2::Pool::builder()
        .connection_timeout(std::time::Duration::from_secs(1))
        .build(manager)
        .expect("could not build connection pool")
}