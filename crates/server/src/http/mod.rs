use poem::middleware::{CatchPanic, NormalizePath, TrailingSlash};
use poem::{get, post, Endpoint, EndpointExt, Route};

use aver_common::http::utils::health::health;
use aver_common::http::utils::log::log;
use aver_common::http::utils::panic::PanicHandler;

pub mod services;
pub mod middlewares;

use crate::http::middlewares::auth::endpoints::create_token;
use crate::http::services::ocm::endpoints::legacy_discovery;
use crate::http::services::ocm::router::ocm;
use crate::http::services::users::router::users;
use crate::http::services::wellknown::router::wellknown;
use crate::models::ApplicationState;
use crate::settings::settings;

pub fn application(state: ApplicationState) -> impl Endpoint {
    Route::new()
        .nest("/", services())
        .with(NormalizePath::new(TrailingSlash::Trim))
        .with(CatchPanic::new().with_handler(PanicHandler::new()))
        .data(state)
        .around(log)
        .boxed()
}

fn services() -> impl Endpoint {
    Route::new()
        .nest("/", root())
        .nest("/.well-known", wellknown())
        .nest(settings().ocm.provider.get_prefix(), ocm())
        .nest("/users", users())
}

fn root() -> impl Endpoint {
    Route::new()
        .at("/ocm-provider", get(legacy_discovery))
        .at("/health", get(health))
        .at("/token", post(create_token))
}
