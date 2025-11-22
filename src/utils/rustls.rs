use crate::api_error::ApiError;
use crate::utils::get_env_or_default;
use actix_web::http::StatusCode;
use rustls::version::{TLS12, TLS13};
use rustls::{ServerConfig, SupportedProtocolVersion};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn open_file(path: &str, key_name: &str) -> Result<BufReader<File>, ApiError> {
    File::open(path).map(BufReader::new).map_err(|error| {
        ApiError::new_log(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error opening the '{key_name}' file: {error}"),
        )
    })
}

pub fn init_rustls() -> Result<Option<ServerConfig>, ApiError> {
    let private_key = get_env_or_default("HTTP_PRIVATE_KEY", "");
    let certificate_chain = get_env_or_default("HTTP_CERTIFICATE_CHAIN", "");

    if private_key.is_empty() && certificate_chain.is_empty() {
        return Ok(None);
    }

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .map_err(|_| {
            ApiError::new_log(
                StatusCode::INTERNAL_SERVER_ERROR,
                "An error occurred while installing rustls.",
            )
        })?;

    let private_key_path = Path::new(&private_key);
    let certificate_chain_path = Path::new(&certificate_chain);
    if !private_key_path.exists() {
        return Err(ApiError::new_log(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("The file in variable 'HTTP_PRIVATE_KEY' does not exist ({private_key})."),
        ));
    }
    if !certificate_chain_path.exists() {
        return Err(ApiError::new_log(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!(
                "The file in variable 'HTTP_CERTIFICATE_CHAIN' does not exist ({certificate_chain})."
            ),
        ));
    }

    let mut key_file = open_file(&private_key, "HTTP_PRIVATE_KEY")?;
    let mut certs_file = open_file(&certificate_chain, "HTTP_CERTIFICATE_CHAIN")?;

    let tls_key = rustls_pemfile::pkcs8_private_keys(&mut key_file)
        .next()
        .ok_or_else(|| {
            ApiError::new_log(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("No private key found in '{private_key}' file."),
            )
        })?
        .map_err(|error| {
            ApiError::new_log(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!(
                    "An error occurred while retrieving the private key ({private_key}): {error}"
                ),
            )
        })?;
    let tls_certs = rustls_pemfile::certs(&mut certs_file)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| {
            ApiError::new_log(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("An error occurred while retrieving the certificate: {error}"),
            )
        })?;

    let tls_versions: Vec<&'static SupportedProtocolVersion> =
        get_env_or_default("TLS_VERSIONS", "TLS13,TLS12")
            .split(',')
            .filter_map(|version| match version.to_lowercase().trim() {
                "tls13" | "tls1.3" => Some(&TLS13),
                "tls12" | "tls1.2" => Some(&TLS12),
                _ => None,
            })
            .collect();

    Ok(Some(
        ServerConfig::builder_with_protocol_versions(&tls_versions)
            .with_no_client_auth()
            .with_single_cert(tls_certs, rustls::pki_types::PrivateKeyDer::Pkcs8(tls_key))
            .map_err(|error| {
                ApiError::new_log(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("An error occurred while creating the rustls config: {error}"),
                )
            })?,
    ))
}
