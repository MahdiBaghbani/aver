use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[oai(rename = "OCM discovery")]
pub struct DiscoveryData {
    pub enabled: bool,
    #[oai(rename = "apiVersion")]
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    #[oai(rename = "endPoint")]
    #[serde(rename = "endPoint")]
    pub end_point: String,
    pub provider: Option<String>,
    #[oai(rename = "resourceTypes")]
    #[serde(rename = "resourceTypes")]
    pub resource_types: Vec<ResourceTypes>,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[oai(rename = "OCM discovery resource types")]
pub struct ResourceTypes {
    pub name: String,
    #[oai(rename = "shareTypes")]
    #[serde(rename = "shareTypes")]
    pub share_types: Vec<String>,
    pub protocols: Protocols,
}

#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[oai(rename = "OCM discovery protocols")]
pub struct Protocols {
    pub webdav: String,
    pub webapp: Option<String>,
    pub datatx: Option<String>,
}
