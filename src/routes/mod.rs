use actix_governor::governor::middleware::NoOpMiddleware;
use actix_governor::{Governor, GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor};
use actix_web::web::Redirect;
use actix_web::{get, web, Responder};
use actix_web_static_files::ResourceFiles;

pub mod document;
pub mod index;
pub mod new;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[get("/favicon.ico")]
async fn favicon() -> actix_web::Result<impl Responder> {
    Ok(Redirect::to("static/assets/pictures/favicon.ico"))
}

pub fn setup_routes(config: &mut web::ServiceConfig) {
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
    let generated_static_ressources = generate();

    config
        .service(favicon)
        .service(ResourceFiles::new("/static", generated_static_ressources))
        .service(web::resource("/").route(web::get().to(index::index)))
        .service(
            web::resource("/new")
                .wrap(Governor::new(&governor_conf))
                .route(web::post().to(new::new)),
        )
        .service(web::resource("/{document_id}").route(web::get().to(document::document)))
        .service(web::resource("/raw/{raw_id}").route(web::get().to(document::document)));
}
