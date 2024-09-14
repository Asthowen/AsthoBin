use crate::api_error::ApiError;
use crate::routes::AsthoBinTemplate;
use actix_web::HttpResponse;
use rinja::Template;

pub async fn index() -> Result<HttpResponse, ApiError> {
    let render: String = AsthoBinTemplate {
        code: None,
        raw_url: None,
    }
    .render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(render))
}
