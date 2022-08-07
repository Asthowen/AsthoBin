use crate::database::models::AsthoBin;
use crate::database::schema::asthobin::dsl as asthobin_dsl;
use crate::util::utils::get_key;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::RunQueryDsl;
use rand::distributions::Alphanumeric;
use rand::distributions::DistString;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn new(
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>,
    bytes: web::Bytes,
    query: HttpRequest,
) -> Result<HttpResponse> {
    let conn: PooledConnection<ConnectionManager<MysqlConnection>> = match pool.get() {
        Ok(pool) => pool,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let document_content: String =
        String::from_utf8(bytes.to_vec()).unwrap_or_else(|_| "".to_owned());
    if document_content.trim().is_empty() {
        let data: &str = r#"{"status": "error","message": "This file is empty."}"#;
        return Ok(HttpResponse::Ok().content_type("text/json").body(data));
    }

    let random_url: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 10);

    let current_time: u64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(value) => value,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    }
    .as_secs();

    let document: AsthoBin = AsthoBin {
        id: random_url.clone(),
        content: document_content,
        time: i64::try_from(current_time).unwrap(),
    };
    diesel::insert_into(asthobin_dsl::asthobin)
        .values(document)
        .execute(&conn)
        .map_err(|err| log::error!("{:?}", err))
        .ok();

    if get_key("LOG_ON_SAVE") == "true" {
        let user_ip: String = query
            .connection_info()
            .realip_remote_addr()
            .unwrap()
            .to_owned();
        log::info!("New code saved with ID: {} - IP: {}.", random_url, user_ip);
    }

    let data: String = r#"{"status": "success", "key": ""#.to_owned() + &random_url + r#""}"#;
    Ok(HttpResponse::Ok()
        .append_header(("Location", format!("/{}", random_url)))
        .content_type("text/json")
        .body(data))
}
