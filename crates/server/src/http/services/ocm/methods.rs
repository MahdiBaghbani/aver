use crate::http::services::ocm::constants::OCM_API_VERSION;
use crate::http::services::ocm::models::{DiscoveryData, DiscoveryResourceTypes};
use crate::settings::models::OcmProvider;

impl DiscoveryData {
    pub fn new(ocm_provider: OcmProvider) -> Self {
        let resource_types: Vec<DiscoveryResourceTypes> = ocm_provider.resource_types.iter()
            .map(|resource_type| resource_type.get_api_payload()).collect();

        DiscoveryData {
            enabled: ocm_provider.enable,
            api_version: OCM_API_VERSION.to_string(),
            endpoint: ocm_provider.get_ocm_endpoint(),
            provider: ocm_provider.provider.clone(),
            resource_types,
            capabilities: ocm_provider.capabilities.get_api_payload(),
        }
    }
}
