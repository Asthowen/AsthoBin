use crate::api_error::ApiError;
use actix_web::HttpResponse;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

pub async fn index() -> Result<HttpResponse, ApiError> {
    let render: String = Index {}.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(render))
}
