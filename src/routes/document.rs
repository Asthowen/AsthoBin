use crate::api_error::ApiError;
use crate::database::postgres::PgPool;
use crate::database::schema::asthobin::dsl as asthobin_dsl;
use crate::routes::AsthoBinTemplate;
use crate::utils::syntect::highlight_string;
use crate::utils::{IGNORED_DOCUMENTS, get_unix_time};
use actix_web::web::{Data, ThinData};
use actix_web::{HttpRequest, HttpResponse};
use askama::Template;
use dashmap::DashMap;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use syntect::highlighting::Theme;
use syntect::parsing::SyntaxSet;

pub async fn document(
    ThinData(pool): ThinData<PgPool>,
    syntect_theme: Data<Theme>,
    syntax_set: Data<SyntaxSet>,
    formated_code_cache: Data<DashMap<String, (String, String, i64)>>,
    query: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let (is_raw, id): (bool, &str) = match (
        query.match_info().get("document_id"),
        query.match_info().get("raw_id"),
    ) {
        (Some(document_id), None) => (false, document_id),
        (None, Some(raw_id)) => (true, raw_id),
        _ => return Ok(HttpResponse::BadRequest().finish()),
    };

    if IGNORED_DOCUMENTS.contains(&id) {
        return Ok(HttpResponse::NotFound().finish());
    }

    let (document, language) = if !is_raw && let Some(element) = formated_code_cache.get(id) {
        let value = element.value();
        (value.0.clone(), value.1.clone())
    } else {
        let Some((content, language)) = asthobin_dsl::asthobin
            .select((asthobin_dsl::content, asthobin_dsl::language))
            .filter(asthobin_dsl::id.eq(&id))
            .first::<(String, String)>(&mut pool.get().await?)
            .await
            .optional()?
        else {
            return Ok(HttpResponse::Found()
                .append_header(("Location", "/"))
                .finish());
        };

        if is_raw {
            (content, language)
        } else {
            let document = highlight_string(&content, &language, syntect_theme, syntax_set)?;

            formated_code_cache.insert(
                id.to_owned(),
                (document.clone(), language.clone(), get_unix_time()?),
            );

            (document, language)
        }
    };

    if is_raw {
        Ok(HttpResponse::Ok().content_type("text/plain").body(document))
    } else {
        let render: String = AsthoBinTemplate {
            code: Some(document),
            raw_url: Some(format_args!("/raw/{id}")),
            language: Some(language),
        }
        .render()?;
        Ok(HttpResponse::Ok().content_type("text/html").body(render))
    }
}
