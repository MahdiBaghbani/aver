use poem::http::header::CONTENT_TYPE;
use poem::http::HeaderMap;
use poem::{
    handler,
    http::StatusCode,
    web::Json,
    web::Redirect,
    IntoResponse,
};

use crate::http::services::ocm::models::{
    DiscoveryData,
    InviteAcceptedRequestData,
    InviteAcceptedResponseData,
};
use crate::settings::settings;

#[handler]
pub async fn discovery() -> impl IntoResponse {
    let discovery_data: DiscoveryData = DiscoveryData::new(
        settings().ocm.provider.clone()
    );

    let mut headers: HeaderMap = HeaderMap::try_with_capacity(1).unwrap();
    headers.insert(CONTENT_TYPE, "application/hal+json".parse().unwrap());

    (StatusCode::OK, headers, Json(discovery_data))
}

#[handler]
pub async fn legacy_discovery() -> Redirect {
    let uri: String = format!("{}/.well-known/ocm", settings().server.get_url());
    Redirect::moved_permanent(uri)
}

#[handler]
pub async fn mfa_capable() -> impl IntoResponse {
    if settings().ocm.provider.capabilities.mfa_capable {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

#[handler]
pub async fn invite_accepted(Json(_request): Json<InviteAcceptedRequestData>) -> impl IntoResponse {
    let resp = InviteAcceptedResponseData {
        user_id: "1".to_string(),
        email: "mahdi-baghbani@azadehafzar.io".to_string(),
        name: "Mahdi Baghbani".to_string(),
    };

    (StatusCode::OK, Json(resp))
}
