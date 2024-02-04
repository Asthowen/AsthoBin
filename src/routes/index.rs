use actix_web::{HttpResponse, Result};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

pub async fn index() -> Result<HttpResponse> {
    let render: String = Index {}.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(render))
}
