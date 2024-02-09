#![allow(clippy::implicit_return, clippy::extra_unused_lifetimes)]
#![deny(clippy::needless_return)]

#[macro_use]
extern crate diesel;

pub mod database;
pub mod routes;
pub mod tasks;
pub mod util;

pub mod api_error;
