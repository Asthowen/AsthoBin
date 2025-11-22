use crate::api_error::ApiError;
use crate::database::mysql::MysqlPool;
use crate::database::schema::asthobin::dsl as asthobin_dsl;
use crate::utils::syntect::highlight_string;
use crate::utils::{get_unix_time, parse_env_or_default};
use actix_web::http::StatusCode;
use actix_web::web::{Data, ThinData};
use actix_web::{HttpRequest, HttpResponse, web};
use dashmap::DashMap;
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use rand::distr::{Alphanumeric, SampleString};
use serde_json::json;
use syntect::highlighting::Theme;
use syntect::parsing::SyntaxSet;

const DEFAULT_SYNTAX: &str = "Plain Text";

pub async fn new(
    ThinData(pool): ThinData<MysqlPool>,
    syntect_theme: Data<Theme>,
    syntax_set: Data<SyntaxSet>,
    formated_code_cache: Data<DashMap<String, (String, String, i64)>>,
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

    let random_url: String = Alphanumeric.sample_string(&mut rand::rng(), 10);
    let time: i64 = get_unix_time()?;
    diesel::insert_into(asthobin_dsl::asthobin)
        .values((
            asthobin_dsl::id.eq(&random_url),
            asthobin_dsl::content.eq(&document_content),
            asthobin_dsl::language.eq(&language),
            asthobin_dsl::time.eq(&time),
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

    let log_on_save: bool = parse_env_or_default("LOG_ON_SAVE", false);
    if log_on_save {
        let connection_info = query.connection_info();
        let user_ip = connection_info.realip_remote_addr().unwrap_or("unknown");
        log::info!(
            "New code saved with ID: {} - IP: {user_ip} - Size: {}o.",
            random_url,
            document_content.len()
        );
    }

    Ok(HttpResponse::Ok()
        .append_header(("Location", format!("/{random_url}")))
        .json(json!({"status": "success", "key": random_url})))
}
