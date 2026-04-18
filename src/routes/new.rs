use actix_web::http::StatusCode;
use actix_web::web::{Bytes, Data, ThinData};
use actix_web::{HttpMessage, HttpRequest, HttpResponse};
use dashmap::DashMap;
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use rand::RngExt;
use serde_json::json;
use syntect::highlighting::Theme;
use syntect::parsing::SyntaxSet;

use crate::api_error::ApiError;
use crate::database::postgres::PgPool;
use crate::database::schema::asthobin;
use crate::utils::syntect::highlight_string;
use crate::utils::unix_timestamp;

const DEFAULT_SYNTAX: &str = "Plain Text";
const ALPHABET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";

pub async fn new(
    query: HttpRequest,
    ThinData(pool): ThinData<PgPool>,
    syntect_theme: Data<Theme>,
    syntax_set: Data<SyntaxSet>,
    formated_code_cache: Data<DashMap<String, (String, String, i64)>>,
    bytes: Bytes,
) -> Result<HttpResponse, ApiError> {
    let document_content: String = String::from_utf8_lossy(&bytes).to_string();
    if document_content.trim().is_empty() {
        return Err(ApiError::new_message(
            StatusCode::BAD_REQUEST,
            "This file is empty.",
        ));
    }

    let language = match query.headers().get("Language") {
        Some(language) => {
            let language = language.to_str().unwrap_or(DEFAULT_SYNTAX);
            if language.contains("sql") {
                "sql"
            } else {
                language
            }
        }
        None => DEFAULT_SYNTAX,
    };

    let random_url: String = (0..10)
        .map(|_| {
            let index = rand::rng().random_range(0..ALPHABET.len());
            ALPHABET[index] as char
        })
        .collect();
    query.extensions_mut().insert::<String>(random_url.clone());

    let time: i64 = unix_timestamp()?;
    diesel::insert_into(asthobin::table)
        .values((
            asthobin::id.eq(&random_url),
            asthobin::content.eq(&document_content),
            asthobin::language.eq(&language),
            asthobin::time.eq(&time),
        ))
        .execute(&mut pool.get().await?)
        .await?;

    formated_code_cache.insert(
        random_url.clone(),
        (
            highlight_string(&document_content, language, syntect_theme, syntax_set)?,
            language.to_owned(),
            time,
        ),
    );

    Ok(HttpResponse::Ok()
        .append_header(("Location", format!("/{random_url}")))
        .json(json!({"status": "success", "key": random_url})))
}
