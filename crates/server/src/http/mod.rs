use poem::middleware::{CatchPanic, NormalizePath, TrailingSlash};
use poem::session::{CookieConfig, RedisStorage, ServerSession};
use poem::{get, post, Endpoint, EndpointExt, Route};
use redis::aio::ConnectionManager;
use redis::Client;

use aver_common::http::utils::health::health;
use aver_common::http::utils::log::log;
use aver_common::http::utils::panic::PanicHandler;

use super::models::ApplicationState;
use super::settings::settings;

pub mod services;
pub mod middlewares;

use self::middlewares::auth::endpoints::create_token;
use self::services::ocm::endpoints::legacy_discovery;
use self::services::ocm::router::ocm;
use self::services::users::router::users;
use self::services::wellknown::router::wellknown;

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
        .nest(settings().ocm.provider.get_prefix(), ocm())
        .nest("/users", users())
}

fn root() -> impl Endpoint {
    Route::new()
        .at("/ocm-provider", get(legacy_discovery))
        .at("/health", get(health))
        .at("/token", post(create_token))
}

pub async fn session() -> ServerSession<RedisStorage<ConnectionManager>> {
    let client: Client = Client::open(settings().session.get_uri()).unwrap();
    let connection: ConnectionManager = ConnectionManager::new(client).await.unwrap();

    ServerSession::new(
        CookieConfig::default(),
        RedisStorage::new(connection),
    )
}
