#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::needless_pass_by_value
)]

pub mod api_error;
pub mod config;
pub mod database;
pub mod middlewares;
pub mod routes;
pub mod tasks;
pub mod utils;
