use crate::router::routes::*;
use actix_files::{Files, NamedFile};
use actix_governor::governor::middleware::NoOpMiddleware;
use actix_governor::{Governor, GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor};
use actix_web::{get, web, Responder, Result};

#[get("/favicon.ico")]
async fn favicon() -> Result<impl Responder> {
    Ok(NamedFile::open("static/assets/pictures/favicon.ico")?)
}

pub fn router(config: &mut web::ServiceConfig) {
    let governor_conf: GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware> =
        GovernorConfigBuilder::default()
            .per_second(
                std::env::var("RATELIMIT_BETWEEN_SAVE")
                    .unwrap_or_else(|_| "2".to_owned())
                    .parse::<u64>()
                    .unwrap_or(2),
            )
            .burst_size(
                std::env::var("RATELIMIT_ALLOWED_BEFORE")
                    .unwrap_or_else(|_| "4".to_owned())
                    .parse::<u32>()
                    .unwrap_or(4),
            )
            .finish()
            .unwrap();

    config
        .service(favicon)
        .service(Files::new("/static", "static"))
        .service(web::resource("/").route(web::get().to(index::index)))
        .service(
            web::resource("/new")
                .wrap(Governor::new(&governor_conf))
                .route(web::post().to(new::new)),
        )
        .service(web::resource("/{document_id}").route(web::get().to(document::document)))
        .service(web::resource("/raw/{raw_id}").route(web::get().to(document::document)));
}
