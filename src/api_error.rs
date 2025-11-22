use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Syntect error: {0}")]
    Syntect(#[from] syntect::Error),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Diesel error: {0}")]
    Diesel(#[from] diesel::result::Error),
    #[error("bb8 error: {0}")]
    Bb8(#[from] diesel_async::pooled_connection::bb8::RunError),
    #[error("Creation pool error: {0}")]
    Bb8Pool(#[from] diesel_async::pooled_connection::PoolError),
    #[error("Askama error: {0}")]
    Askama(#[from] askama::Error),
    #[error("Configuration error: {0}")]
    Confik(#[from] confik::Error),
    #[error("String parsing error: {0}")]
    ParseStringError(#[from] std::string::FromUtf8Error),
    #[error("SystemTimeError error: {0}")]
    SystemTimeError(#[from] std::time::SystemTimeError),
    #[error("Conversion error: {0}")]
    Infallible(#[from] std::convert::Infallible),
    #[error("Integer conversion error: {0}")]
    TryFromIntError(#[from] std::num::TryFromIntError),
    #[error("Custom error: {log_message:?} (code: {status_code})")]
    Custom {
        status_code: StatusCode,
        log_message: Option<String>,
        http_message: Option<String>,
    },
}

impl ApiError {
    pub fn new_log_internal<S: Into<String>>(log_message: S) -> Self {
        Self::new_log(StatusCode::INTERNAL_SERVER_ERROR, log_message)
    }

    pub fn new_log<S: Into<String>>(status_code: StatusCode, log_message: S) -> Self {
        Self::Custom {
            status_code,
            log_message: Some(log_message.into()),
            http_message: None,
        }
    }

    pub fn new_message<S: Into<String>>(status_code: StatusCode, http_message: S) -> Self {
        Self::Custom {
            status_code,
            log_message: None,
            http_message: Some(http_message.into()),
        }
    }

    pub fn new_all<S1: Into<String>, S2: Into<String>>(
        status_code: StatusCode,
        log_message: S1,
        http_message: S2,
    ) -> Self {
        Self::Custom {
            status_code,
            log_message: Some(log_message.into()),
            http_message: Some(http_message.into()),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status_code, log_message, http_message) = match self {
            Self::Custom {
                status_code,
                log_message,
                http_message,
            } => (*status_code, log_message.clone(), http_message.clone()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(self.to_string()),
                None,
            ),
        };

        if let Some(log_message) = &log_message {
            log::error!("{log_message}");
        }

        if let Some(http_message) = &http_message {
            HttpResponse::build(status_code)
                .json(json!({"status": "failed", "message": http_message}))
        } else {
            HttpResponse::build(status_code).finish()
        }
    }
}
