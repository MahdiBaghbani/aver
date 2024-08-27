pub mod services;
pub mod utils;
pub mod middlewares;

use poem::middleware::{CatchPanic, NormalizePath, TrailingSlash};
use poem::{get, Endpoint, EndpointExt, Route};

use crate::http::services::wellknown::router::wellknown;
use crate::http::utils::health::health;
use crate::http::utils::log::log;
use crate::http::utils::panic::PanicHandler;

pub fn application() -> impl Endpoint {
    Route::new()
        .nest("/", services())
        .with(NormalizePath::new(TrailingSlash::Trim))
        .with(CatchPanic::new().with_handler(PanicHandler::new()))
        .around(log)
        .boxed()
}

fn services() -> impl Endpoint {
    Route::new()
        .nest("/", root())
        .nest("/.well-known", wellknown())
}

fn root() -> impl Endpoint {
    Route::new()
        .at("/health", get(health))
}
