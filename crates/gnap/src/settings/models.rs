use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub log: Log,
    pub server: Server,
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
    pub mode: ServerMode,
}

#[derive(Debug, Deserialize)]
pub enum ServerMode {
    Development,
    Production,
}
