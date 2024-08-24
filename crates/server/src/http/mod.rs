pub mod services;
pub mod utils;
pub mod middlewares;

use poem::middleware::{CatchPanic, NormalizePath, TrailingSlash};
use poem::{get, post, Endpoint, EndpointExt, Route};

use crate::http::middlewares::auth::endpoints::create_token;
use crate::http::services::ocm::endpoints::legacy_discovery;
use crate::http::services::ocm::router::ocm;
use crate::http::services::wellknown::router::wellknown;
use crate::http::utils::health::health;
use crate::http::utils::log::log;
use crate::http::utils::panic::PanicHandler;
use crate::settings::methods::settings;

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
        .nest(settings().ocm.provider.prefix.clone(), ocm())
}

fn root() -> impl Endpoint {
    Route::new()
        .at("/ocm-provider", get(legacy_discovery))
        .at("/health", get(health))
        .at("/token", post(create_token))
}
