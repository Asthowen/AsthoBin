use actix_web::{HttpResponse, Result};
use crate::util::utils::get_key;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    base_url: String
}

pub async fn index() -> Result<HttpResponse> {
    let render: String = Index {
        base_url: get_key("BASE_URL")
    }
        .render()
        .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(render))
}