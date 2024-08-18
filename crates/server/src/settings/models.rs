use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub log: Log,
    pub server: Server,
    pub ocm_provider: OcmProvider,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub domain: String,
    pub scheme: String,
    pub ip: String,
    pub port: String,
}

#[derive(Debug, Deserialize)]
pub struct OcmProvider {
    pub enable: bool,
    // default: ocm; the prefix URL where the OCM API is served.
    pub prefix: String,
    // this host's full URL. if it's not configured, it is assumed OCM is not available.
    pub endpoint: String,
    // default aver; a friendly name that defines this service.
    pub provider: Option<String>,
    pub resource_types: Vec<OcmProviderResourceTypes>,
    pub capabilities: OcmProviderCapabilities,
}

#[derive(Debug, Deserialize)]
pub struct OcmProviderResourceTypes {
    pub name: String,
    pub share_types: OcmProviderShareTypes,
    pub protocols: OcmProviderProtocols,
}

#[derive(Debug, Deserialize)]
pub struct OcmProviderShareTypes {
    pub user: bool,
    pub group: bool,
    pub federation: bool,
}

#[derive(Debug, Deserialize)]
pub struct OcmProviderProtocols {
    // default: /dav; the root URL of the WebDAV endpoint to serve OCM shares.
    pub webdav: String,
    // default: /app the root URL to serve Web apps via OCM.
    pub webapp: Option<String>,
    pub datatx: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OcmProviderCapabilities {
    pub mfa_capable: bool,
    pub notifications: bool,
    pub invite_accepted: bool,
}
