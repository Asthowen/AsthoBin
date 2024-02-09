use crate::api_error::ApiError;
use crate::database::models::AsthoBin;
use crate::database::mysql::{MysqlPool, MysqlPooled};
use crate::database::schema::asthobin::dsl as asthobin_dsl;
use crate::util::utils::{get_key, IGNORED_DOCUMENTS};
use actix_web::dev::ConnectionInfo;
use actix_web::{web, HttpRequest, HttpResponse};
use askama::Template;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::cell::Ref;

#[derive(Template)]
#[template(path = "code.html")]
struct Code {
    code: String,
    raw_url: String,
}

pub async fn document(
    pool: web::Data<MysqlPool>,
    query: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let (is_doc, id): (bool, String) =
        if let Some(document_id) = query.match_info().get("document_id") {
            (true, document_id.parse()?)
        } else {
            (false, query.match_info().get("raw_id").unwrap().parse()?)
        };

    if IGNORED_DOCUMENTS.contains(&id.as_str()) {
        return Ok(HttpResponse::NotFound().finish());
    }

    let mut conn: MysqlPooled = pool.get().await?;

    let document: Option<AsthoBin> = asthobin_dsl::asthobin
        .filter(asthobin_dsl::id.like(&id))
        .first::<AsthoBin>(&mut conn)
        .await
        .optional()?;

    if let Some(document) = document {
        let log_on_access: String =
            std::env::var("LOG_ON_ACCESS").unwrap_or_else(|_| "false".to_owned());
        if log_on_access == "true" {
            let connection_info: Ref<ConnectionInfo> = query.connection_info();
            let current_url: String = format!(
                "{}://{}{}",
                connection_info.scheme(),
                connection_info.host(),
                query.path()
            );
            let user_ip: String = query
                .connection_info()
                .realip_remote_addr()
                .unwrap_or("unknown")
                .to_owned();
            log::info!(
                "Access to the code present at: {} - IP: {}.",
                current_url,
                user_ip
            );
        }
        if is_doc {
            let render: String = Code {
                code: document.content,
                raw_url: format!("{}raw/{}", get_key("BASE_URL"), id),
            }
            .render()?;
            Ok(HttpResponse::Ok().content_type("text/html").body(render))
        } else {
            Ok(HttpResponse::Ok()
                .content_type("text/plain")
                .body(document.content))
        }
    } else {
        Ok(HttpResponse::Found()
            .append_header(("Location", get_key("BASE_URL")))
            .finish())
    }
}
