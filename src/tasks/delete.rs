use crate::database::mysql::{MysqlPool, MysqlPooled};
use crate::database::schema::asthobin::dsl as asthobin_dsl;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn delete(pool: &MysqlPool) {
    let mut conn: MysqlPooled = match pool.get().await {
        Ok(pool) => pool,
        Err(_) => return,
    };

    let delete_time: u64 = std::env::var("DELETE_TIME")
        .unwrap_or_else(|_| "604_800".to_owned())
        .parse::<u64>()
        .unwrap_or(604800);

    let current_time: u64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(value) => value,
        Err(_) => return,
    }
    .as_secs();

    diesel::delete(asthobin_dsl::asthobin)
        .filter(asthobin_dsl::time.lt(i64::try_from(current_time - delete_time).unwrap()))
        .execute(&mut conn)
        .await
        .map_err(|err| log::error!("{:?}", err))
        .ok();
}
