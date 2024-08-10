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
    pub provider: String,
    // default: /dav/ocm; the root URL of the WebDAV endpoint to serve OCM shares.
    pub webdav_root: String,
    // default: /app/ocm; the root URL to serve Web apps via OCM.
    pub webapp_root: String,
    // default: false; whether web apps are enabled in OCM shares.
    pub webapp_enable: bool,
    // default: false; whether data transfers are enabled in OCM shares.
    pub datatx_enable: bool,
}
