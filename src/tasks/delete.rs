use crate::api_error::ApiError;
use crate::config::Config;
use crate::database::mysql::MysqlPool;
use crate::database::schema::asthobin::dsl as asthobin_dsl;
use crate::utils::get_unix_time;
use actix_web::web::Data;
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;

pub async fn delete(pool: &MysqlPool, config: &Data<Config>) -> Result<(), ApiError> {
    let current_time: i64 = get_unix_time()?;

    diesel::delete(asthobin_dsl::asthobin)
        .filter(asthobin_dsl::time.lt(current_time - config.delete_time))
        .execute(&mut pool.get().await?)
        .await?;

    Ok(())
}
