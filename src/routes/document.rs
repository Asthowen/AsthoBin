use actix_web::HttpResponse;
use actix_web::web::{Data, Path, ThinData};
use askama::Template;
use dashmap::DashMap;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use syntect::highlighting::Theme;
use syntect::parsing::SyntaxSet;

use super::AsthoBinTemplate;
use crate::api_error::ApiError;
use crate::database::postgres::PgPool;
use crate::database::schema::asthobin;
use crate::utils::syntect::highlight_string;
use crate::utils::{IGNORED_DOCUMENTS, unix_timestamp};

#[derive(Deserialize)]
pub struct PathDocument {
    pub document_id: Option<String>,
    pub raw_id: Option<String>,
}

pub async fn document(
    path: Path<PathDocument>,
    ThinData(pool): ThinData<PgPool>,
    formated_code_cache: Data<DashMap<String, (String, String, i64)>>,
    syntax_set: Data<SyntaxSet>,
    syntect_theme: Data<Theme>,
) -> Result<HttpResponse, ApiError> {
    let (is_raw, id): (bool, &str) = match (&path.document_id, &path.raw_id) {
        (Some(document_id), None) => (false, document_id),
        (None, Some(raw_id)) => (true, raw_id),
        _ => return Ok(HttpResponse::BadRequest().finish()),
    };

    if IGNORED_DOCUMENTS.contains(&id) {
        return Ok(HttpResponse::NotFound().finish());
    }

    let (document, language) = if !is_raw && let Some(element) = formated_code_cache.get(id) {
        let (document, language, _) = element.value();
        (document.clone(), language.clone())
    } else {
        let Some((content, language)) = asthobin::table
            .select((asthobin::content, asthobin::language))
            .filter(asthobin::id.eq(&id))
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
                (document.clone(), language.clone(), unix_timestamp()?),
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
