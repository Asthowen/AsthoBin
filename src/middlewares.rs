use crate::config::Config;
use actix_web::body::MessageBody;
use actix_web::dev::{ConnectionInfo, ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::web::Data;
use actix_web::{Error, HttpMessage};
use std::cell::Ref;

pub async fn log(
    config: Data<Config>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let is_save = config.log_on_save && req.path().eq("/new");

    let res = next.call(req).await?;

    {
        let connection_info: Ref<ConnectionInfo> = res.request().connection_info();
        let current_url: String = format!(
            "{}://{}{}",
            connection_info.scheme(),
            connection_info.host(),
            res.request().path()
        );
        let user_ip = connection_info.realip_remote_addr().unwrap_or("unknown");
        if is_save {
            log::info!(
                "New code saved with ID: {} - IP: {user_ip}",
                match res.request().extensions().get::<String>() {
                    Some(code) => code.as_str(),
                    None => "unknown",
                }
            );
        } else {
            log::info!("Access to the code present at: {current_url} - IP: {user_ip}",);
        }
    }

    Ok(res)
}
