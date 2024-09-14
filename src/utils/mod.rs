pub mod logger;
#[cfg(feature = "rustls")]
pub mod rustls;

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

pub fn get_env_or_default(key_name: &str, default_value: &str) -> String {
    std::env::var(key_name).unwrap_or_else(|_| default_value.to_owned())
}

pub fn parse_env_or_default<T: FromStr>(key_name: &str, default_value: T) -> T
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    std::env::var(key_name)
        .ok()
        .and_then(|value| value.parse::<T>().ok())
        .unwrap_or(default_value)
}
pub fn exit_if_key_not_exist(key_name: &str) {
    if std::env::var(key_name).is_err() {
        log::error!("The key {key_name} does not exist in the .env file.");
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
