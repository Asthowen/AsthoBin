use confik::Configuration;

#[derive(Debug, PartialEq, Configuration)]
pub struct Config {
    #[confik(default = "127.0.0.1")]
    pub host: String,
    #[confik(default = 8080u16)]
    pub port: u16,
    #[confik(secret, default = "")]
    pub database_url: String,
    pub database_url_file: Option<String>,
    #[confik(default = "")]
    pub cors_origin: String,
    #[confik(default = false)]
    pub log_on_access: bool,
    #[confik(default = false)]
    pub log_on_save: bool,
    #[confik(default = 604800)]
    pub delete_time: i64,
    #[confik(default = 2u64)]
    pub ratelimit_between_save: u64,
    #[confik(default = 4u32)]
    pub ratelimit_allowed_before: u32,
    pub pool_max_connections: Option<u32>,
    pub pool_min_reserved_connections: Option<u32>,
    pub pool_connection_timeout: Option<u64>,
}
