use poem::middleware::{CatchPanic, NormalizePath, TrailingSlash};
use poem::session::{CookieConfig, RedisStorage, ServerSession};
use poem::{Endpoint, EndpointExt, Route};
use redis::aio::ConnectionManager;
use redis::Client;

use super::models::ApplicationState;
use super::settings::settings;
use super::utils::log::log;
use super::utils::panic::PanicHandler;

pub mod services;
pub mod middlewares;

use self::services::auth::router::auth;
use self::services::ocm::router::ocm;
use self::services::root::router::root;
use self::services::users::router::users;
use self::services::wellknown::router::wellknown;

pub enum EndpointType {
    Api,
    Ssr,
}

pub async fn application(state: ApplicationState) -> impl Endpoint {
    let session: ServerSession<RedisStorage<ConnectionManager>> = session().await;
    let catch_panic: CatchPanic<PanicHandler> = CatchPanic::new().with_handler(PanicHandler::new());

    Route::new()
        .nest("/", services())
        .with(NormalizePath::new(TrailingSlash::Trim))
        .with(session)
        .with(catch_panic)
        .data(state)
        .around(log)
        .boxed()
}

fn services() -> impl Endpoint {
    Route::new()
        .nest("/", root())
        .nest("/.well-known", wellknown())
        .nest("/api", api())
        .nest("/auth", auth(EndpointType::Ssr))
        .nest(settings().ocm.provider.get_prefix(), ocm())
        .nest("/users", users(EndpointType::Ssr))
}

fn api() -> impl Endpoint {
    Route::new()
        .nest("/auth", auth(EndpointType::Api))
        .nest("/users", users(EndpointType::Api))
}

pub async fn session() -> ServerSession<RedisStorage<ConnectionManager>> {
    let client: Client = Client::open(settings().session.get_uri()).unwrap();
    let connection: ConnectionManager = ConnectionManager::new(client).await.unwrap();

    ServerSession::new(
        CookieConfig::default(),
        RedisStorage::new(connection),
    )
}
