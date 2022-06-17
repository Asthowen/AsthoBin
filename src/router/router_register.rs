use actix_web::{web, Result, Responder, get};
use actix_files::{Files, NamedFile};
use crate::router::routes::*;

#[get("/favicon.ico")]
async fn favicon() -> Result<impl Responder> {
    Ok(NamedFile::open("static/assets/pictures/favicon.ico")?)
}

pub fn router(config: &mut web::ServiceConfig) {
    config
        .service(favicon)
        .service(Files::new("/static", "static"))
        .service(web::resource("/").route(web::get().to(index::index)))
        .service(web::resource("/new").route(web::post().to(new::new)))
        .service(web::resource("/{document_id}").route(web::get().to(document::document)));
}