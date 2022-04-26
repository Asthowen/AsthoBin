use actix_web::{HttpResponse, Result};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    base_url: String
}

pub async fn index() -> Result<HttpResponse> {
    let s = Index {
        base_url: std::env::var("BASE_URL").unwrap()
    }
        .render()
        .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}