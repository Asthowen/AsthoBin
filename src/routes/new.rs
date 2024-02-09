use crate::api_error::ApiError;
use crate::database::models::AsthoBin;
use crate::database::mysql::{MysqlPool, MysqlPooled};
use crate::database::schema::asthobin::dsl as asthobin_dsl;
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};
use diesel_async::RunQueryDsl;
use rand::distributions::Alphanumeric;
use rand::distributions::DistString;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn new(
    pool: web::Data<MysqlPool>,
    bytes: web::Bytes,
    query: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let mut conn: MysqlPooled = pool.get().await?;

    let document_content: String = String::from_utf8(bytes.to_vec())?;
    if document_content.trim().is_empty() {
        return Err(ApiError::new_message(
            StatusCode::BAD_REQUEST,
            "This file is empty.",
        ));
    }

    let random_url: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 10);

    let current_time: u64 = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    let document: AsthoBin = AsthoBin {
        id: random_url,
        content: document_content,
        time: i64::try_from(current_time)?,
    };
    diesel::insert_into(asthobin_dsl::asthobin)
        .values(&document)
        .execute(&mut conn)
        .await?;

    let log_on_save: String = std::env::var("LOG_ON_SAVE").unwrap_or_else(|_| "false".to_owned());
    if log_on_save == "true" {
        let user_ip: String = query
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_owned();
        log::info!("New code saved with ID: {} - IP: {}.", document.id, user_ip);
    }

    Ok(HttpResponse::Ok()
        .append_header(("Location", format!("/{}", document.id)))
        .json(json!({"status": "success", "key": document.id})))
}
