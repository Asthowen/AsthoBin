use crate::config::Config;
#[cfg(debug_assertions)]
use actix_files::Files;
use actix_files::NamedFile;
use actix_governor::governor::middleware::NoOpMiddleware;
use actix_governor::{Governor, GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor};
use actix_web::web::Data;
use actix_web::{Responder, get, web};
#[cfg(not(debug_assertions))]
use actix_web_static_files::ResourceFiles;
use askama::Template;
use std::fmt::Arguments;

pub mod document;
pub mod index;
pub mod new;

#[cfg(not(debug_assertions))]
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub mod generated_assets {
    include!(concat!(env!("OUT_DIR"), "/generated_assets.rs"));
}

#[derive(Default, Template)]
#[template(path = "index.html")]
struct AsthoBinTemplate<'a> {
    code: Option<String>,
    raw_url: Option<Arguments<'a>>,
    language: Option<String>,
}

#[get("/favicon.ico")]
async fn favicon() -> actix_web::Result<impl Responder> {
    Ok(NamedFile::open("static/assets/pictures/favicon.png")?)
}

pub fn setup(config: Data<Config>, service_config: &mut web::ServiceConfig) {
    let governor_conf: GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware> =
        GovernorConfigBuilder::default()
            .seconds_per_request(config.ratelimit_between_save)
            .burst_size(config.ratelimit_allowed_before)
            .finish()
            .unwrap_or_else(|| {
                log::error!("Invalid Governor configuration.");
                std::process::exit(9);
            });

    #[cfg(not(debug_assertions))]
    service_config.service(web::scope("/assets").service(ResourceFiles::new("", generate())));

    #[cfg(debug_assertions)]
    service_config.service(web::scope("/assets").service(Files::new("", "static/assets/")));

    service_config
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
