use crate::ocm::models::{DiscoveryData, InviteAcceptedRequestData, InviteAcceptedResponseData};
use crate::settings::methods::settings;
use poem::{
    handler,
    http::StatusCode,
    web::Json,
    web::Redirect,
    Response,
};

#[handler]
pub async fn discovery() -> Response {
    let discovery_data: DiscoveryData = DiscoveryData::new(
        settings().ocm_provider.clone()
    );

    // discovery_data
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
pub async fn invite_accepted(_req: Json<InviteAcceptedRequestData>) -> Response {
    let resp = InviteAcceptedResponseData {
        user_id: "1".to_string(),
        email: "mahdi-baghbani@azadehafzar.io".to_string(),
        name: "Mahdi Baghbani".to_string(),
    };

    // discovery_data
    let json: String = serde_json::to_string(&resp).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .content_type("application/json")
        .body(json)
}
