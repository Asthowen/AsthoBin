use crate::api_error::ApiError;
use crate::database::mysql::MysqlPool;
use crate::database::schema::asthobin::dsl as asthobin_dsl;
use crate::util::utils::parse_env_or_default;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn delete(pool: MysqlPool) -> Result<(), ApiError> {
    let delete_time: u64 = parse_env_or_default("DELETE_TIME", 604_800);
    let current_time: u64 = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    diesel::delete(asthobin_dsl::asthobin)
        .filter(asthobin_dsl::time.lt(i64::try_from(current_time - delete_time)?))
        .execute(&mut pool.get().await?)
        .await?;

    Ok(())
}
