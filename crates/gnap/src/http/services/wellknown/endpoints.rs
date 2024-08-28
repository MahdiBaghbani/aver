use poem::{
    handler,
    http::StatusCode,
    web::Json,
    IntoResponse,
};
use tracing::trace;

use crate::http::services::wellknown::models::OpenIDConfiguration;
use crate::settings::settings;

#[handler]
pub async fn openid_config() -> impl IntoResponse {
    trace!("get openid_config");

    let url: String = settings().server.get_url();
    let issuer: String = format!("{}", url);
    let authorization_endpoint: String = format!("{}/gnap/auth", url);
    let token_endpoint: String = format!("{}/gnap/token", url);
    let userinfo_endpoint: String = format!("{}/gnap/userinfo", url);
    let jwks_uri: String = format!("{}/gnap/jwks", url);

    let response: OpenIDConfiguration = OpenIDConfiguration::new(
        issuer,
        authorization_endpoint,
        token_endpoint,
        userinfo_endpoint,
        jwks_uri,
    );

    (StatusCode::OK, Json(response))
}