use crate::api_error::ApiError;
use crate::routes::AsthoBinTemplate;
use actix_web::HttpResponse;
use askama::Template;

pub async fn index() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(AsthoBinTemplate::default().render()?))
}
