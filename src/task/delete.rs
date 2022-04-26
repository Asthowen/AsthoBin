use crate::database::schema::asthobin::dsl as asthobin_dsl;
use diesel::r2d2::{ConnectionManager, Pool};
use std::time::{SystemTime, UNIX_EPOCH};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;


pub async fn delete(pool: &Pool<ConnectionManager<MysqlConnection>>) {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let current_time: u64 =  SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    diesel::delete(asthobin_dsl::asthobin
        .filter(
            asthobin_dsl::time.lt(i64::try_from(current_time - 604800).unwrap())
        )
    )
        .execute(&conn)
        .map_err(|err| log::error!("{:?}", err))
        .ok();
}