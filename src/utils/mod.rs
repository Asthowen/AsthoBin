pub mod logger;
pub mod syntect;

use crate::api_error::ApiError;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub const WAIT_TWO_SECONDS: Duration = Duration::from_secs(2);
pub const WAIT_ONE_HOUR: Duration = Duration::from_secs(3600);

pub const IGNORED_DOCUMENTS: [&str; 5] = [
    "robots.txt",
    "sitemap.xml",
    "security.txt",
    ".well-known/robots.txt",
    ".htaccess",
];

pub fn get_unix_time() -> Result<i64, ApiError> {
    Ok(i64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| ApiError::new_log_internal(error.to_string()))?
            .as_secs(),
    )?)
}
