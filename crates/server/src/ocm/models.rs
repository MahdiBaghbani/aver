use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct OcmError {
    pub message: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DiscoveryData {
    pub enabled: bool,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    #[serde(rename = "endPoint")]
    pub end_point: String,
    pub provider: Option<String>,
    #[serde(rename = "resourceTypes")]
    pub resource_types: Vec<ResourceTypes>,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResourceTypes {
    pub name: String,
    #[serde(rename = "shareTypes")]
    pub share_types: Vec<String>,
    pub protocols: Protocols,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Protocols {
    pub webdav: String,
    pub webapp: Option<String>,
    pub datatx: Option<String>,
}
