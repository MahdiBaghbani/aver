use poem::{
    handler,
    http::StatusCode,
    web::Json,
    IntoResponse,
};
use tracing::trace;

use crate::http::services::wellknown::models::{GnapOptions, OpenIDConfiguration};
use crate::settings::settings;

#[handler]
pub async fn openid_config() -> impl IntoResponse {
    trace!("get openid_config");

    let url: String = settings().server.get_url();
    let response: OpenIDConfiguration = OpenIDConfiguration::new(url);

    (StatusCode::OK, Json(response))
}

#[handler]
pub async fn gnap_config() -> impl IntoResponse {
    trace!("get gnap_config");

    let url: String = settings().server.get_url();
    let response: GnapOptions = GnapOptions::new(url);

    (StatusCode::OK, Json(response))
}
