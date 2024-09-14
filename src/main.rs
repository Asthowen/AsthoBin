use actix_cors::Cors;
use actix_web::web::ThinData;
use actix_web::{web, App, HttpResponse, HttpServer};
use asthobin::database::mysql;
use asthobin::database::mysql::MysqlPool;
use asthobin::routes;
use asthobin::tasks::delete;
use asthobin::utils::logger;
#[cfg(feature = "rustls")]
use asthobin::utils::rustls::init_rustls;
use asthobin::utils::{
    exit_if_keys_not_exist, get_env_or_default, parse_env_or_default, WAIT_ONE_HOUR,
};

fn main() {
    dotenvy::dotenv().ok();
    logger::init();

    #[cfg(debug_assertions)]
    exit_if_keys_not_exist(&["VITE_DEV_URL"]);

    exit_if_keys_not_exist(&["DATABASE_URL"]);

    let worker_threads_number: usize = parse_env_or_default("ACTIX_WORKER_THREADS_NUMBER", 4);

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
    let worker_threads_number: usize = parse_env_or_default("ACTIX_WORKER_THREADS_NUMBER", 4);
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

    let http_server_bind;

    #[cfg(feature = "rustls")]
    {
        match init_rustls() {
            Ok(Some(config)) => {
                http_server_bind = http_server.bind_rustls_0_23(format!("{host}:{port}"), config);
            }
            Err(error) => {
                log::error!("An error occurred when initializing the rustls config: {error}. Start without SSL certificate.");
                http_server_bind = http_server.bind(format!("{host}:{port}"));
            }
            _ => {
                http_server_bind = http_server.bind(format!("{host}:{port}"));
            }
        }
    }

    #[cfg(not(feature = "rustls"))]
    {
        http_server_bind = http_server.bind(format!("{host}:{port}"));
    }

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
        });
}
