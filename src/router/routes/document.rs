use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use crate::database::schema::asthobin::dsl as asthobin_dsl;
use actix_web::{web, HttpResponse, Result, HttpRequest};
use crate::database::models::AsthoBin;
use diesel::mysql::MysqlConnection;
use crate::util::utils::get_key;
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
    let conn: PooledConnection<ConnectionManager<MysqlConnection>> = match pool.get() {
        Ok(pool) => pool,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish())
    };

    let document: Option<AsthoBin> = asthobin_dsl::asthobin
        .filter(
            asthobin_dsl::id
                .like(document_id)
        )
        .first::<AsthoBin>(&conn)
        .optional()
        .map_err(|e| (http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        .unwrap();

    if document.is_some() {
        let code: String = document.unwrap().content;
        let render: String = Code {
            code: html_escape::encode_script(&code).to_string()
        }
            .render()
            .unwrap();
        Ok(HttpResponse::Ok().content_type("text/html").body(render))
    } else {
        Ok(HttpResponse::Found().append_header(("Location", get_key("BASE_URL"))).finish())
    }
}