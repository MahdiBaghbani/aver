use poem::middleware::{CatchPanic, NormalizePath, TrailingSlash};
use poem::{get, Endpoint, EndpointExt, Route};

use crate::http::endpoints::health;
use crate::http::models::PanicHandler;
use crate::http::services::ocm::endpoints::legacy_discovery;
use crate::http::services::ocm::router::ocm;
use crate::http::services::wellknown::router::wellknown;
use crate::settings::methods::settings;
use crate::utils::log::log;

pub fn routes() -> impl Endpoint {
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
        .nest(settings().ocm_provider.prefix.clone(), ocm())
}

fn root() -> impl Endpoint {
    Route::new()
        .at("/ocm-provider", get(legacy_discovery))
        .at("/health", get(health))
}