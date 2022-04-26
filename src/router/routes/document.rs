use crate::database::schema::asthobin::dsl as asthobin_dsl;
use actix_web::{web, HttpResponse, Result, HttpRequest};
use diesel::r2d2::{ConnectionManager, Pool};
use crate::database::models::AsthoBin;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use askama::Template;
use actix_web::http;

#[derive(Template)]
#[template(path = "code.html")]
struct Code {
    code: String
}

pub async fn document(pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>, query: HttpRequest) -> Result<HttpResponse> {
    let document_id: String = query.match_info().get("document_id").unwrap().parse().unwrap();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let exists = asthobin_dsl::asthobin
        .filter(
            asthobin_dsl::id
                .like(document_id)
        )
        .first::<AsthoBin>(&conn)
        .optional()
        .map_err(|e| (http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        .unwrap();

    if exists.is_some() {
        let code: String = exists.unwrap().content;
        let s = Code {
            code: html_escape::encode_text(&code).to_string()
        }
            .render()
            .unwrap();
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    } else {
        Ok(HttpResponse::Found().append_header(("Location", std::env::var("BASE_URL").unwrap())).finish())
    }
}