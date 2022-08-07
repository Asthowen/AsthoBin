use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};
use asthobin::database::mysql;
use asthobin::router::router_register::router;
use asthobin::tasks::delete;
use asthobin::util::logger::init_logger;
use asthobin::util::utils::exit_if_keys_not_exist;
use std::sync::Arc;

fn main() {
    dotenv::dotenv().ok();
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
    let host: String = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port: String = std::env::var("PORT").unwrap_or_else(|_| "8080".to_owned());
    let cors_origin: String = std::env::var("CORS_ORIGIN").unwrap_or_else(|_| "".to_owned());
    if cors_origin.is_empty() {
        log::warn!("The CORS_ORIGIN environment variable has not been defined, so CORS has been completely disabled (this does not prevent the server from working, but it is a security issue).");
    }
    let pool: mysql::MysqlPool = mysql::get_pool();

    let pool_arc: Arc<mysql::MysqlPool> = Arc::new(pool.clone());
    tokio::task::spawn(async move {
        let pool_arc: &mysql::MysqlPool = &*pool_arc;

        loop {
            delete::delete(pool_arc).await;
            tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
        }
    });

    log::info!("Start server on {}:{}...", host, port);

    HttpServer::new(move || {
        let mut cors: Cors = Cors::default()
            .allowed_methods(vec!["GET", "POST"])
            .allow_any_header()
            .max_age(3600);

        if cors_origin.is_empty() || cors_origin == "*" {
            cors = cors.allow_any_origin();
        } else {
            cors = cors.allowed_origin(cors_origin.as_str());
        }

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .default_service(web::to(HttpResponse::Ok))
            .configure(router)
            .wrap(cors)
    })
    .workers(8)
    .bind(format!("{}:{}", host, port))
    .unwrap_or_else(|_| panic!("Couldn't bind to port {}", port))
    .run()
    .await
    .unwrap()
}
