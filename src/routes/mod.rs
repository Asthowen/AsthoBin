use crate::util::utils::parse_env_or_default;
#[cfg(debug_assertions)]
use actix_files::Files;
use actix_files::NamedFile;
use actix_governor::governor::middleware::NoOpMiddleware;
use actix_governor::{Governor, GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor};
use actix_web::{get, web, Responder};
#[cfg(not(debug_assertions))]
use actix_web_static_files::ResourceFiles;
use askama::Template;
use std::fmt::Arguments;

pub mod document;
pub mod index;
pub mod new;

#[cfg(not(debug_assertions))]
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Template)]
#[template(path = "index.html")]
struct AsthoBinTemplate<'a> {
    code: Option<String>,
    raw_url: Option<Arguments<'a>>,
}

#[get("/favicon.ico")]
async fn favicon() -> actix_web::Result<impl Responder> {
    Ok(NamedFile::open("static/assets/pictures/favicon.png")?)
}

pub fn setup(config: &mut web::ServiceConfig) {
    let governor_conf: GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware> =
        GovernorConfigBuilder::default()
            .per_second(parse_env_or_default("RATELIMIT_BETWEEN_SAVE", 2))
            .burst_size(parse_env_or_default("RATELIMIT_ALLOWED_BEFORE", 4))
            .finish()
            .unwrap_or_else(|| {
                log::error!("Invalid Governor configuration.");
                std::process::exit(9);
            });

    #[cfg(not(debug_assertions))]
    config.service(web::scope("/assets").service(ResourceFiles::new("", generate())));

    #[cfg(debug_assertions)]
    config.service(web::scope("/assets").service(Files::new("", "static/assets/")));

    config
        .service(favicon)
        .service(web::resource("/").route(web::get().to(index::index)))
        .service(
            web::resource("/new")
                .wrap(Governor::new(&governor_conf))
                .route(web::post().to(new::new)),
        )
        .service(
            web::resource(["/{document_id}", "/raw/{raw_id}"])
                .route(web::get().to(document::document)),
        );
}
