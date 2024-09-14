use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use diesel_async::pooled_connection::bb8::RunError;
use serde_json::json;
use std::convert::Infallible;
use std::fmt;
use std::num::TryFromIntError;
use std::string::FromUtf8Error;
use std::time::SystemTimeError;

#[derive(Debug)]
pub struct ApiError {
    pub status_code: StatusCode,
    pub log_message: Option<String>,
    pub http_message: Option<String>,
}

impl ApiError {
    pub const fn new(
        status_code: StatusCode,
        log_message: Option<String>,
        http_message: Option<String>,
    ) -> Self {
        Self {
            status_code,
            log_message,
            http_message,
        }
    }

    pub fn new_log<S: Into<String>>(status_code: StatusCode, log_message: S) -> Self {
        Self {
            status_code,
            log_message: Some(log_message.into()),
            http_message: None,
        }
    }

    pub fn new_message<S: Into<String>>(status_code: StatusCode, http_message: S) -> Self {
        Self {
            status_code,
            log_message: None,
            http_message: Some(http_message.into()),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(log_message) = &self.log_message {
            f.write_str(log_message)
        } else if let Some(http_message) = &self.http_message {
            f.write_str(http_message)
        } else {
            f.write_str("No error message.")
        }
    }
}

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> Self {
        Self::new_log(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("A diesel error has occurred: {error}"),
        )
    }
}

impl From<RunError> for ApiError {
    fn from(error: RunError) -> Self {
        Self::new_log(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("A bb8 (pool) error has occurred: {error}"),
        )
    }
}

impl From<rinja::Error> for ApiError {
    fn from(error: rinja::Error) -> Self {
        Self::new_log(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("A rinja error has occurred: {error}"),
        )
    }
}

impl From<FromUtf8Error> for ApiError {
    fn from(_: FromUtf8Error) -> Self {
        Self::new_message(
            StatusCode::BAD_REQUEST,
            "Text cannot be converted to UTF-8.",
        )
    }
}

impl From<TryFromIntError> for ApiError {
    fn from(error: TryFromIntError) -> Self {
        Self::new_message(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("An error has occurred when try convert an integer: {error}",),
        )
    }
}

impl From<Infallible> for ApiError {
    fn from(error: Infallible) -> Self {
        Self::new_log(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("An 'Infallible' error has occurred: {error}"),
        )
    }
}

impl From<SystemTimeError> for ApiError {
    fn from(error: SystemTimeError) -> Self {
        Self::new_log(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("An 'SystemTimeError' error has occurred: {error}"),
        )
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        if let Some(log_message) = &self.log_message {
            log::error!("{}", log_message);
        }

        if let Some(http_message) = &self.http_message {
            HttpResponse::build(self.status_code)
                .json(json!({"status": "failed", "message": http_message}))
        } else {
            HttpResponse::build(self.status_code).finish()
        }
    }
}
