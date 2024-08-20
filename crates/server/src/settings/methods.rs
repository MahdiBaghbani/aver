use config::{Config, ConfigError, Environment, File};
use std::env;
use tracing::debug;

use crate::http::services::ocm::models::{
    DiscoveryProtocols,
    DiscoveryResourceTypes,
};
use crate::settings::models::{
    OcmProvider,
    OcmProviderCapabilities,
    OcmProviderProtocols,
    OcmProviderResourceTypes,
    OcmProviderShareTypes,
    Server,
    Settings,
};
use crate::settings::SETTINGS;

pub fn settings() -> &'static Settings {
    SETTINGS.get().expect("config init")
}

pub fn init() {
    let settings: Settings = Settings::new().unwrap();

    // show loaded settings.
    debug!("{:#?}", settings);

    SETTINGS.set(settings).expect("Somehow Darth Sidious has returned!");
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let file_path: String = env::var("AVER_CONFIG_PATH").unwrap();

        let settings: Config = Config::builder()
            .add_source(File::with_name(&file_path))
            // add in settings from the environment (with a prefix of APP)
            // eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("AVER"))
            .build()?;

        // deserialize and thus freeze the entire configuration.
        settings.try_deserialize()
    }
}

impl Server {
    pub fn get_url(&self) -> String {
        format!("{}://{}:{}", self.scheme, self.domain, self.port)
    }
    pub fn get_tcp_bind(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

impl Clone for OcmProvider {
    fn clone(&self) -> Self {
        OcmProvider {
            enable: self.enable,
            prefix: self.prefix.clone(),
            endpoint: self.endpoint.clone(),
            provider: self.provider.clone(),
            resource_types: self.resource_types.clone(),
            capabilities: self.capabilities.clone(),
        }
    }
}

impl OcmProvider {
    pub fn get_ocm_endpoint(&self) -> String {
        format!(
            "{}/{}",
            self.endpoint.clone(),
            self.prefix.clone()
        )
    }
}

impl Clone for OcmProviderResourceTypes {
    fn clone(&self) -> Self {
        OcmProviderResourceTypes {
            name: self.name.clone(),
            share_types: self.share_types.clone(),
            protocols: self.protocols.clone(),
        }
    }
}

impl OcmProviderResourceTypes {
    pub fn get_api_payload(&self) -> DiscoveryResourceTypes {
        DiscoveryResourceTypes {
            name: self.name.clone(),
            share_types: self.share_types.get_api_payload(),
            protocols: self.protocols.get_api_payload(),
        }
    }
}

impl Clone for OcmProviderShareTypes {
    fn clone(&self) -> Self {
        OcmProviderShareTypes {
            user: self.user,
            group: self.group,
            federation: self.federation,
        }
    }
}

impl OcmProviderShareTypes {
    pub fn get_api_payload(&self) -> Vec<String> {
        let mut share_types: Vec<String> = Vec::new();

        if self.user {
            share_types.push("user".to_string());
        }

        if self.group {
            share_types.push("group".to_string());
        }

        if self.federation {
            share_types.push("federation".to_string());
        }

        share_types
    }
}

impl Clone for OcmProviderProtocols {
    fn clone(&self) -> Self {
        OcmProviderProtocols {
            webdav: self.webdav.clone(),
            webapp: self.webapp.clone(),
            datatx: self.datatx.clone(),
        }
    }
}

impl OcmProviderProtocols {
    pub fn get_api_payload(&self) -> DiscoveryProtocols {
        DiscoveryProtocols {
            webdav: self.webdav.clone(),
            webapp: self.webapp.clone(),
            datatx: self.datatx.clone(),
        }
    }
}

impl Clone for OcmProviderCapabilities {
    fn clone(&self) -> Self {
        OcmProviderCapabilities {
            mfa_capable: self.mfa_capable,
            notifications: self.notifications,
            invite_accepted: self.invite_accepted,
        }
    }
}

impl OcmProviderCapabilities {
    pub fn get_api_payload(&self) -> Vec<String> {
        let mut capabilities: Vec<String> = vec!["/shares".to_string()];

        if self.mfa_capable {
            capabilities.push("/mfa-capable".to_string());
        }

        if self.notifications {
            capabilities.push("/notifications".to_string());
        }

        if self.invite_accepted {
            capabilities.push("/invite-accepted".to_string());
        }

        capabilities
    }
}
