use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use crate::database::schema::asthobin::dsl as asthobin_dsl;
use std::time::{SystemTime, UNIX_EPOCH};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;


pub async fn delete(pool: &Pool<ConnectionManager<MysqlConnection>>) {
    let conn: PooledConnection<ConnectionManager<MysqlConnection>> = match pool.get() {
        Ok(pool) => pool,
        Err(_) => return
    };

    let delete_time: u64 = std::env::var("DELETE_TIME")
        .unwrap_or_else(|_| String::from("604800"))
        .parse::<u64>()
        .unwrap_or_else(|_| 604800);

    let current_time: u64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(value) => value,
        Err(_) => return
    }.as_secs();

    diesel::delete(asthobin_dsl::asthobin
        .filter(
            asthobin_dsl::time.lt(i64::try_from(current_time - delete_time).unwrap())
        )
    )
        .execute(&conn)
        .map_err(|err| log::error!("{:?}", err))
        .ok();
}