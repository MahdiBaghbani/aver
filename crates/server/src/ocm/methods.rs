use crate::ocm::constants::{OCM_API_VERSION, OCM_CAPABILITIES};
use crate::ocm::models::{DiscoveryData, DiscoveryProtocols, DiscoveryResourceTypes};
use crate::settings::models::OcmProvider;

impl DiscoveryData {
    pub fn new(ocm_provider: OcmProvider) -> Self {
        let protocols: DiscoveryProtocols = DiscoveryProtocols {
            webdav: ocm_provider.webdav_root.clone(),
            webapp: if ocm_provider.webapp_enable {
                Some(ocm_provider.webapp_root.clone())
            } else {
                None
            },
            datatx: if ocm_provider.datatx_enable {
                Some(ocm_provider.webdav_root.clone())
            } else {
                None
            },
        };

        let resource_types: DiscoveryResourceTypes = DiscoveryResourceTypes {
            // so far we only support `file`.
            name: "file".to_string(),
            // so far we only support `user`.
            share_types: vec!["user".to_string()],
            protocols,
        };

        let endpoint: String = format!(
            "{}/{}",
            ocm_provider.endpoint.clone(),
            ocm_provider.prefix.clone()
        );

        DiscoveryData {
            enabled: ocm_provider.enable,
            api_version: OCM_API_VERSION.to_string(),
            end_point: endpoint,
            provider: Some(ocm_provider.provider.clone()),
            resource_types: vec![resource_types],
            capabilities: OCM_CAPABILITIES.map(String::from).to_vec(),
        }
    }
}
