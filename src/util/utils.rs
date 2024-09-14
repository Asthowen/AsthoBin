use openssl::ssl::SslVersion;
use std::str::FromStr;
use std::time::Duration;

pub const WAIT_TWO_SECONDS: Duration = Duration::from_secs(2);
pub const WAIT_ONE_HOUR: Duration = Duration::from_secs(3600);

pub const IGNORED_DOCUMENTS: [&str; 5] = [
    "robots.txt",
    "sitemap.xml",
    "security.txt",
    ".well-known/robots.txt",
    ".htaccess",
];

pub fn get_env_or_default(var: &str, default_value: &str) -> String {
    std::env::var(var).unwrap_or_else(|_| default_value.to_owned())
}

pub fn parse_env_or_default<T: FromStr>(var: &str, default_value: T) -> T
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    std::env::var(var)
        .ok()
        .and_then(|val| val.parse::<T>().ok())
        .unwrap_or(default_value)
}
pub fn exit_if_key_not_exist(key: &str) {
    if std::env::var(key).is_err() {
        log::error!("The key {key} does not exist in the .env file.");
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

pub fn map_to_ssl_version(ssl_version: &str) -> Option<SslVersion> {
    match ssl_version {
        "ssl3" => Some(SslVersion::SSL3),
        "tls1" => Some(SslVersion::TLS1),
        "tls1.1" => Some(SslVersion::TLS1_1),
        "tls1.2" => Some(SslVersion::TLS1_2),
        "tls1.3" => Some(SslVersion::TLS1_3),
        _ => None,
    }
}
