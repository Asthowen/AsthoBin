pub mod logger;
pub mod syntect;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::api_error::ApiError;

pub const WAIT_TWO_SECONDS: Duration = Duration::from_secs(2);
pub const WAIT_ONE_HOUR: Duration = Duration::from_hours(1);

pub const IGNORED_DOCUMENTS: [&str; 5] = [
    "robots.txt",
    "sitemap.xml",
    "security.txt",
    ".well-known/robots.txt",
    ".htaccess",
];

pub fn unix_timestamp() -> Result<i64, ApiError> {
    Ok(i64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| ApiError::new_log_internal(error.to_string()))?
            .as_secs(),
    )?)
}
