use crate::database::schema::asthobin::dsl as asthobin_dsl;
use rand::{distributions::Alphanumeric, Rng};
use diesel::r2d2::{ConnectionManager, Pool};
use actix_web::{web, HttpResponse, Result};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::database::models::AsthoBin;
use diesel::mysql::MysqlConnection;
use diesel::RunQueryDsl;


pub async fn new(pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>, bytes: actix_web::web::Bytes) -> Result<HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let document_content: String = String::from_utf8(bytes.to_vec()).unwrap_or_else(|_| String::from(""));
    if document_content.trim() == "" {
        let data: &str = r#"{"status": "error","message": "This file is empty."}"#;
        return Ok(HttpResponse::Ok().content_type("text/json").body(data));
    }

    let random_url: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    let current_time: u64 =  SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards").as_secs();

    let document: AsthoBin = AsthoBin {
        id: random_url.clone(), content: document_content, time: i64::try_from(current_time).unwrap()
    };
    diesel::insert_into(asthobin_dsl::asthobin)
        .values(document)
        .execute(&conn)
        .expect("Error inserting document");
    let data: &str = r#"{"status": "success"}"#;
    Ok(HttpResponse::Ok().append_header(("Location", format!("/{}", random_url))).content_type("text/json").body(data))
}