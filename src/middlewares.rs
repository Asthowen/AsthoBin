use actix_web::body::MessageBody;
use actix_web::dev::{ConnectionInfo, ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::web::Data;
use actix_web::{Error, HttpMessage};

use crate::config::Config;

pub async fn log(
    config: Data<Config>,
    connection_info: ConnectionInfo,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let save_request = config.log_on_save && req.path().eq("/new");

    let res = next.call(req).await?;

    let user_ip = connection_info.realip_remote_addr().unwrap_or("unknown");
    if save_request {
        log::info!(
            "Code saved: ID={}, IP={user_ip}",
            match res.request().extensions().get::<String>() {
                Some(code) => code.as_str(),
                None => "unknown",
            }
        );
    } else {
        let current_url = format_args!(
            "{}://{}{}",
            connection_info.scheme(),
            connection_info.host(),
            res.request().path()
        );
        log::info!("Code accessed: URL={current_url}, IP={user_ip}");
    }

    Ok(res)
}
