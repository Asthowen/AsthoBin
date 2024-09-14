use crate::api_error::ApiError;
use crate::database::models::AsthoBin;
use crate::database::mysql::MysqlPool;
use crate::database::schema::asthobin::dsl as asthobin_dsl;
use crate::utils::parse_env_or_default;
use actix_web::http::StatusCode;
use actix_web::web::ThinData;
use actix_web::{web, HttpRequest, HttpResponse};
use diesel_async::RunQueryDsl;
use rand::distributions::Alphanumeric;
use rand::distributions::DistString;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn new(
    ThinData(pool): ThinData<MysqlPool>,
    bytes: web::Bytes,
    query: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let document_content: String = String::from_utf8(bytes.to_vec())?;
    if document_content.trim().is_empty() {
        return Err(ApiError::new_message(
            StatusCode::BAD_REQUEST,
            "This file is empty.",
        ));
    }

    let random_url: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 10);
    let time: i64 = i64::try_from(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())?;

    let document: AsthoBin = AsthoBin {
        id: random_url,
        content: document_content,
        time,
    };
    diesel::insert_into(asthobin_dsl::asthobin)
        .values(&document)
        .execute(&mut pool.get().await?)
        .await?;

    let log_on_save: bool = parse_env_or_default("LOG_ON_SAVE", false);

    if log_on_save {
        let connection_info = query.connection_info();
        let user_ip = connection_info.realip_remote_addr().unwrap_or("unknown");
        log::info!(
            "New code saved with ID: {} - IP: {user_ip} - Size: {}o.",
            document.id,
            document.content.len()
        );
    }

    Ok(HttpResponse::Ok()
        .append_header(("Location", format!("/{}", document.id)))
        .json(json!({"status": "success", "key": document.id})))
}
