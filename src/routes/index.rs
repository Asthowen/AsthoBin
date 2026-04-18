use actix_web::HttpResponse;
use askama::Template;

use super::AsthoBinTemplate;
use crate::api_error::ApiError;

pub async fn index() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(AsthoBinTemplate::default().render()?))
}
