use openssl::ssl::SslVersion;
use std::time::Duration;

pub const WAIT_TWO_SECONDS: Duration = Duration::from_secs(2);
pub const WAIT_ONE_HOUR: Duration = Duration::from_secs(3600);

pub fn exit_if_key_not_exist(key: &str) {
    if std::env::var(key).is_err() {
        log::error!("The key {} does not exist in the .env file.", key);
        std::process::exit(1);
    }
}
pub fn exit_if_keys_not_exist(keys: &[&str]) {
    for key in keys {
        exit_if_key_not_exist(key);
    }
}
pub fn get_key(key_name: &str) -> String {
    std::env::var(key_name).unwrap()
}

pub fn map_to_ssl_version(ssl_version: String) -> Option<SslVersion> {
    match ssl_version.as_str() {
        "ssl3" => Some(SslVersion::SSL3),
        "tls1" => Some(SslVersion::TLS1),
        "tls1.1" => Some(SslVersion::TLS1_1),
        "tls1.2" => Some(SslVersion::TLS1_2),
        "tls1.3" => Some(SslVersion::TLS1_3),
        _ => None,
    }
}
