use actix_web::web::Data;
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;

use crate::api_error::ApiError;
use crate::config::Config;
use crate::database::postgres::PgPool;
use crate::database::schema::asthobin;
use crate::utils::unix_timestamp;

pub async fn delete(pool: &PgPool, config: &Data<Config>) -> Result<(), ApiError> {
    let current_time: i64 = unix_timestamp()?;

    diesel::delete(asthobin::table)
        .filter(asthobin::time.lt(current_time - config.delete_time))
        .execute(&mut pool.get().await?)
        .await?;

    Ok(())
}
