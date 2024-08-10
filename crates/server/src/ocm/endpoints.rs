use poem_openapi::{payload::Json, OpenApi};

use crate::ocm::models::{DiscoveryData, DiscoveryResponse};
use crate::settings::models::OcmProvider;

pub struct Ocm;

#[OpenApi]
impl Ocm {
    #[oai(path = "/.well-known/ocm", method = "get")]
    pub async fn discovery(&self) -> DiscoveryResponse {
        let ocm_provider: OcmProvider = OcmProvider {
            enable: true,
            prefix: "ocm".to_string(),
            endpoint: "https://mamad.docker".to_string(),
            provider: "aver".to_string(),
            webdav_root: "/dav/ocm".to_string(),
            webapp_root: "/app/ocm".to_string(),
            webapp_enable: true,
            datatx_enable: true,
        };

        let discovery_data: DiscoveryData = DiscoveryData::new(
            ocm_provider
        );

        DiscoveryResponse::Ok(Json(discovery_data))
    }
}