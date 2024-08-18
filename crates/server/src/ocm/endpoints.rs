use poem::{
    handler,
    http::StatusCode,
    web::Json,
    web::Redirect,
    Response,
};

use crate::ocm::models::{DiscoveryData, InviteAcceptedRequestData, InviteAcceptedResponseData};
use crate::settings::methods::settings;

#[handler]
pub async fn discovery() -> Response {
    let discovery_data: DiscoveryData = DiscoveryData::new(
        settings().ocm_provider.clone()
    );

    let json: String = serde_json::to_string(&discovery_data).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .content_type("application/hal+json")
        .body(json)
}

#[handler]
pub async fn legacy_discovery() -> Redirect {
    let uri: String = format!("{}/.well-known/ocm", settings().server.get_url());
    Redirect::moved_permanent(uri)
}

#[handler]
pub async fn mfa_capable() -> Response {
    let status_code: StatusCode = if settings().ocm_provider.capabilities.mfa_capable {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    };

    Response::builder()
        .status(status_code)
        .finish()
}

#[handler]
pub async fn invite_accepted(_req: Json<InviteAcceptedRequestData>) -> Response {
    let resp = InviteAcceptedResponseData {
        user_id: "1".to_string(),
        email: "mahdi-baghbani@azadehafzar.io".to_string(),
        name: "Mahdi Baghbani".to_string(),
    };

    let json: String = serde_json::to_string(&resp).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .content_type("application/json")
        .body(json)
}
