use actix_cors::Cors;
use actix_web::web::{Data, ThinData};
use actix_web::{App, HttpResponse, HttpServer, web};
use asthobin::api_error::ApiError;
use asthobin::config::Config;
use asthobin::database::postgres;
use asthobin::database::postgres::PgPool;
use asthobin::routes::setup;
use asthobin::tasks::delete;
use asthobin::utils::WAIT_ONE_HOUR;
use asthobin::utils::{get_unix_time, logger};
use confik::{Configuration, EnvSource};
use dashmap::DashMap;
use std::path::Path;
use syntect::highlighting::{Color, ThemeSet};
use syntect::parsing::SyntaxSet;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    logger::init();

    if let Err(error) = start().await {
        if let ApiError::Custom { log_message, .. } = error {
            if let Some(log_message) = log_message {
                log::error!("{log_message}");
            } else {
                log::error!("An unknown error has occurred.");
            }
        } else {
            log::error!("{error}");
        }
        std::process::exit(1);
    }

    Ok(())
}

async fn start() -> Result<(), ApiError> {
    let config = {
        let mut config = Config::builder()
            .override_with(EnvSource::new().allow_secrets())
            .try_build()?;

        if let Some(database_url_file) = &config.database_url_file {
            let path = Path::new(database_url_file);
            if !path.exists() {
                return Err(ApiError::new_log_internal(
                    "The file provided through the 'DATABASE_URL_FILE' variable does not exist.",
                ));
            }
            let content = tokio::fs::read_to_string(path).await?.trim().to_owned();
            if content.is_empty() {
                return Err(ApiError::new_log_internal(
                    "The file provided through the 'DATABASE_URL_FILE' variable is empty.",
                ));
            }
            config.database_url = content;
        } else if config.database_url.is_empty() {
            return Err(ApiError::new_log_internal(
                "Neither the 'DATABASE_URL' variable nor the 'DATABASE_URL_FILE' variable is defined.",
            ));
        }

        Data::new(config)
    };

    if config.cors_origin.is_empty() {
        log::warn!(
            "The CORS_ORIGIN environment variable has not been defined (or defined to *), so CORS has been completely disabled (this does not prevent the server from working, but it is a security issue)."
        );
    }

    let formated_code_cache: Data<DashMap<String, (String, String, i64)>> =
        Data::new(DashMap::new());

    postgres::run_migration(&config.database_url).await?;

    let pool: PgPool = postgres::get_pool(&config).await?;

    let pool_clone: PgPool = pool.clone();
    let config_clone = Data::clone(&config);
    let formated_code_cache_clone = Data::clone(&formated_code_cache);
    tokio::task::spawn(async move {
        loop {
            if let Err(error) = delete::delete(&pool_clone, &config_clone).await {
                log::error!("An error has occurred while executing the deletion task: {error}");
            }

            if let Ok(unix_time) = get_unix_time() {
                formated_code_cache_clone.retain(|_, (_, _, ttl)| *ttl + 3600 > unix_time);
            }

            tokio::time::sleep(WAIT_ONE_HOUR).await;
        }
    });

    let mut theme = ThemeSet::load_defaults()
        .themes
        .remove("base16-ocean.dark")
        .unwrap_or_else(|| {
            log::error!("The theme 'base16-ocean.dark' is not present.",);
            std::process::exit(9);
        });
    theme.settings.background = Some(Color {
        r: 255,
        g: 255,
        b: 255,
        a: 0,
    });
    let theme_data = Data::new(theme);
    let syntax_set = Data::new(SyntaxSet::load_defaults_newlines());

    log::info!(
        "Start actix-web server on {}:{}...",
        config.host,
        config.port
    );

    let server_host = (config.host.clone(), config.port);
    Ok(HttpServer::new(move || {
        let mut cors: Cors = Cors::default()
            .allowed_methods(["GET", "POST"])
            .allow_any_header()
            .max_age(3600);
        cors = if config.cors_origin.is_empty() || config.cors_origin == "*" {
            cors.allow_any_origin()
        } else {
            cors.allowed_origin(config.cors_origin.as_str())
        };

        App::new()
            .app_data(ThinData(pool.clone()))
            .app_data(Data::clone(&config))
            .app_data(Data::clone(&theme_data))
            .app_data(Data::clone(&syntax_set))
            .app_data(Data::clone(&formated_code_cache))
            .default_service(web::to(HttpResponse::Ok))
            .configure(|service_config| setup(Data::clone(&config), service_config))
            .wrap(cors)
    })
    .bind(server_host)?
    .run()
    .await?)
}
