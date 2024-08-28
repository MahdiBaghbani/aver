use poem::middleware::{CatchPanic, NormalizePath, TrailingSlash};
use poem::{get, Endpoint, EndpointExt, Route};

use aver_common::http::utils::health::health;
use aver_common::http::utils::log::log;
use aver_common::http::utils::panic::PanicHandler;

pub mod services;
pub mod middlewares;

use crate::http::services::wellknown::router::wellknown;

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
