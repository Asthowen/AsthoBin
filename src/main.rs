use actix_cors::Cors;
use actix_web::web::ThinData;
use actix_web::{web, App, HttpResponse, HttpServer};
use asthobin::database::mysql;
use asthobin::database::mysql::MysqlPool;
use asthobin::routes;
use asthobin::tasks::delete;
use asthobin::utils::logger;
use asthobin::utils::{
    exit_if_keys_not_exist, get_env_or_default, map_to_ssl_version, parse_env_or_default,
    WAIT_ONE_HOUR,
};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use std::path::Path;

fn main() {
    dotenvy::dotenv().ok();
    logger::init();

    #[cfg(debug_assertions)]
    exit_if_keys_not_exist(&["VITE_DEV_URL"]);

    exit_if_keys_not_exist(&["DATABASE_URL", "BASE_URL"]);

    let worker_threads_number: usize = parse_env_or_default("ACTIX_WORKER_THREADS_NUMBER", 8);

    actix_web::rt::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(worker_threads_number)
            .thread_name("main-tokio")
            .build()
            .unwrap()
    })
    .block_on(async_main());
}

async fn async_main() {
    let worker_threads_number: usize = parse_env_or_default("ACTIX_WORKER_THREADS_NUMBER", 8);
    let host: String = get_env_or_default("HOST", "127.0.0.1");
    let port: String = get_env_or_default("PORT", "8080");
    let cors_origin: String = get_env_or_default("CORS_ORIGIN", "");
    if cors_origin.is_empty() {
        log::warn!("The CORS_ORIGIN environment variable has not been defined, so CORS has been completely disabled (this does not prevent the server from working, but it is a security issue).");
    }
    let pool: MysqlPool = mysql::get_pool().await;
    mysql::run_migration().await;

    let pool_clone: MysqlPool = pool.clone();
    tokio::task::spawn(async move {
        loop {
            if let Err(error) = delete::delete(pool_clone.clone()).await {
                log::error!("An error has occurred while executing the deletion task: {error}");
            }

            tokio::time::sleep(WAIT_ONE_HOUR).await;
        }
    });

    log::info!("Start actix-web server on {host}:{port}...");

    let http_server = HttpServer::new(move || {
        let mut cors: Cors = Cors::default()
            .allowed_methods(["GET", "POST"])
            .allow_any_header()
            .max_age(3600);

        cors = if cors_origin.is_empty() || cors_origin == "*" {
            cors.allow_any_origin()
        } else {
            cors.allowed_origin(cors_origin.as_str())
        };

        App::new()
            .app_data(ThinData(pool.clone()))
            .default_service(web::to(HttpResponse::Ok))
            .configure(routes::setup)
            .wrap(cors)
    })
    .workers(worker_threads_number);

    let http_private_key: String = get_env_or_default("HTTP_PRIVATE_KEY", "");
    let http_certificate_chain: String = get_env_or_default("HTTP_CERTIFICATE_CHAIN", "");
    let http_server_bind = if !http_private_key.is_empty()
        && !http_certificate_chain.is_empty()
        && Path::new(&http_private_key).exists()
        && Path::new(&http_certificate_chain).exists()
    {
        let ssl_file_type: String = get_env_or_default("SSL_FILE_TYPE", "PEM");
        let ssl_ca_file: String = get_env_or_default("SSL_CA_FILE", "");

        let ssl_protocol_min_version: String = get_env_or_default("SSL_PROTOCOL_MIN_VERSION", "");
        let ssl_protocol_max_version: String = get_env_or_default("SSL_PROTOCOL_MAX_VERSION", "");

        let mut builder: SslAcceptorBuilder =
            SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

        if Path::new(&ssl_ca_file).exists() {
            builder.set_ca_file(&ssl_ca_file).ok();
        }
        builder
            .set_min_proto_version(map_to_ssl_version(&ssl_protocol_min_version.to_lowercase()))
            .ok();
        builder
            .set_max_proto_version(map_to_ssl_version(&ssl_protocol_max_version.to_lowercase()))
            .ok();

        builder
            .set_private_key_file(
                http_private_key,
                match ssl_file_type.to_lowercase().as_str() {
                    "pem" => SslFiletype::PEM,
                    "asn1" => SslFiletype::ASN1,
                    &_ => SslFiletype::PEM,
                },
            )
            .unwrap();
        builder
            .set_certificate_chain_file(http_certificate_chain)
            .unwrap();
        http_server.bind_openssl(format!("{host}:{port}"), builder)
    } else {
        http_server.bind(format!("{host}:{port}"))
    };

    http_server_bind
        .unwrap_or_else(|error| {
            log::error!(
                "Couldn't bind AsthoBin to {}:{}. Error: {}",
                host,
                port,
                error
            );
            std::process::exit(9);
        })
        .run()
        .await
        .unwrap_or_else(|error| {
            log::error!(
                "Couldn't bind AsthoBin to {}:{}. Error: {}",
                host,
                port,
                error
            );
            std::process::exit(9);
        })
}
