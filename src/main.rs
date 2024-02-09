use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};
use asthobin::database::mysql;
use asthobin::routes::setup_routes;
use asthobin::tasks::delete;
use asthobin::util::logger::init_logger;
use asthobin::util::utils::{exit_if_keys_not_exist, map_to_ssl_version, WAIT_ONE_HOUR};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use std::path::Path;
use std::sync::Arc;

fn main() {
    dotenvy::dotenv().ok();
    init_logger();

    exit_if_keys_not_exist(&["DATABASE_URL", "BASE_URL"]);

    let worker_threads_number: usize = std::env::var("ACTIX_WORKER_THREADS_NUMBER")
        .unwrap_or_else(|_| "8".to_owned())
        .parse::<usize>()
        .unwrap_or(8);

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
    let worker_threads_number: usize = std::env::var("ACTIX_WORKER_THREADS_NUMBER")
        .unwrap_or_else(|_| "8".to_owned())
        .parse::<usize>()
        .unwrap_or(8);
    let host: String = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port: String = std::env::var("PORT").unwrap_or_else(|_| "8080".to_owned());
    let cors_origin: String = std::env::var("CORS_ORIGIN").unwrap_or_else(|_| "".to_owned());
    if cors_origin.is_empty() {
        log::warn!("The CORS_ORIGIN environment variable has not been defined, so CORS has been completely disabled (this does not prevent the server from working, but it is a security issue).");
    }
    let pool: mysql::MysqlPool = mysql::get_pool().await;
    mysql::run_migration().await;

    let pool_arc: Arc<mysql::MysqlPool> = Arc::new(pool.clone());
    tokio::task::spawn(async move {
        let pool_arc: &mysql::MysqlPool = &pool_arc;

        loop {
            delete::delete(pool_arc).await;
            tokio::time::sleep(WAIT_ONE_HOUR).await;
        }
    });

    log::info!("Start server on {}:{}...", host, port);

    let http_server = HttpServer::new(move || {
        let mut cors: Cors = Cors::default()
            .allowed_methods(vec!["GET", "POST"])
            .allow_any_header()
            .max_age(3600);

        cors = if cors_origin.is_empty() || cors_origin == "*" {
            cors.allow_any_origin()
        } else {
            cors.allowed_origin(cors_origin.as_str())
        };

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .default_service(web::to(HttpResponse::Ok))
            .configure(setup_routes)
            .wrap(cors)
    })
    .workers(worker_threads_number);

    let http_private_key: String =
        std::env::var("HTTP_PRIVATE_KEY").unwrap_or_else(|_| "".to_owned());
    let http_certificate_chain: String =
        std::env::var("HTTP_CERTIFICATE_CHAIN").unwrap_or_else(|_| "".to_owned());
    let http_server_bind = if !http_private_key.is_empty()
        && !http_certificate_chain.is_empty()
        && Path::new(&http_private_key).exists()
        && Path::new(&http_certificate_chain).exists()
    {
        let ssl_file_type: String =
            std::env::var("SSL_FILE_TYPE").unwrap_or_else(|_| "PEM".to_owned());
        let ssl_ca_file: String = std::env::var("SSL_CA_FILE").unwrap_or_else(|_| "".to_owned());
        let ssl_protocol_min_version: String =
            std::env::var("SSL_PROTOCOL_MIN_VERSION").unwrap_or_else(|_| "".to_owned());
        let ssl_protocol_max_version: String =
            std::env::var("SSL_PROTOCOL_MAX_VERSION").unwrap_or_else(|_| "".to_owned());

        let mut builder: SslAcceptorBuilder =
            SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

        if Path::new(&ssl_ca_file).exists() {
            builder.set_ca_file(&ssl_ca_file).ok();
        }
        builder
            .set_min_proto_version(map_to_ssl_version(ssl_protocol_min_version.to_lowercase()))
            .ok();
        builder
            .set_max_proto_version(map_to_ssl_version(ssl_protocol_max_version.to_lowercase()))
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
        http_server.bind_openssl(format!("{}:{}", host, port), builder)
    } else {
        http_server.bind(format!("{}:{}", host, port))
    };

    http_server_bind
        .unwrap_or_else(|e| {
            log::error!(
                "Couldn't bind AsthoBin to {}:{}. Error: {}",
                host,
                port,
                e.to_string()
            );
            std::process::exit(9);
        })
        .run()
        .await
        .unwrap_or_else(|e| {
            log::error!(
                "Couldn't bind AsthoBin to {}:{}. Error: {}",
                host,
                port,
                e.to_string()
            );
            std::process::exit(9);
        })
}
