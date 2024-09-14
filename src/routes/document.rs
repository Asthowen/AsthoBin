use crate::api_error::ApiError;
use crate::database::mysql::MysqlPool;
use crate::database::schema::asthobin::dsl as asthobin_dsl;
use crate::routes::AsthoBinTemplate;
use crate::util::utils::{get_key, parse_env_or_default, IGNORED_DOCUMENTS};
use actix_web::dev::ConnectionInfo;
use actix_web::web::ThinData;
use actix_web::{HttpRequest, HttpResponse};
use askama::Template;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::cell::Ref;

pub async fn document(
    ThinData(pool): ThinData<MysqlPool>,
    query: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let (is_doc, id): (bool, &str) = match (
        query.match_info().get("document_id"),
        query.match_info().get("raw_id"),
    ) {
        (Some(document_id), None) => (true, document_id),
        (None, Some(raw_id)) => (false, raw_id),
        _ => return Ok(HttpResponse::BadRequest().finish()),
    };

    if IGNORED_DOCUMENTS.contains(&id) {
        return Ok(HttpResponse::NotFound().finish());
    }

    let document_opt: Option<String> = asthobin_dsl::asthobin
        .select(asthobin_dsl::content)
        .filter(asthobin_dsl::id.eq(&id))
        .first::<String>(&mut pool.get().await?)
        .await
        .optional()?;
    let document = match document_opt {
        Some(document) => document,
        None => {
            return Ok(HttpResponse::Found()
                .append_header(("Location", get_key("BASE_URL")))
                .finish())
        }
    };

    let log_on_access: bool = parse_env_or_default("LOG_ON_ACCESS", false);
    if log_on_access {
        let connection_info: Ref<ConnectionInfo> = query.connection_info();
        let current_url: String = format!(
            "{}://{}{}",
            connection_info.scheme(),
            connection_info.host(),
            query.path()
        );
        let user_ip = connection_info.realip_remote_addr().unwrap_or("unknown");
        log::info!("Access to the code present at: {current_url} - IP: {user_ip}.",);
    }
    if is_doc {
        let render: String = AsthoBinTemplate {
            code: Some(document),
            raw_url: Some(format_args!("{}raw/{}", get_key("BASE_URL"), id)),
        }
        .render()?;
        Ok(HttpResponse::Ok().content_type("text/html").body(render))
    } else {
        Ok(HttpResponse::Ok().content_type("text/plain").body(document))
    }
}
