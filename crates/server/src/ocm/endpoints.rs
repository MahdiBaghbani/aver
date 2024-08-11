use poem::{handler, web::Redirect};
use poem_openapi::{payload::Json, OpenApi};

use crate::ocm::models::{DiscoveryData, DiscoveryResponse};
use crate::settings::methods::settings;

pub struct OcmDiscovery;

#[OpenApi]
impl OcmDiscovery {
    #[oai(path = "/.well-known/ocm", method = "get")]
    pub async fn discovery(&self) -> DiscoveryResponse {
        let discovery_data: DiscoveryData = DiscoveryData::new(
            settings().ocm_provider.clone()
        );

        DiscoveryResponse::Ok(Json(discovery_data))
    }
}

#[handler]
pub async fn legacy_discovery() -> Redirect {
    let uri: String = format!("{}/.well-known/ocm", settings().server.get_url());
    Redirect::moved_permanent(uri)
}
