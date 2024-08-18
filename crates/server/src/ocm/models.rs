use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct OcmError {
    pub message: String,
    pub validation_errors: Option<Vec<OcmValidationError>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct OcmValidationError {
    pub name: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DiscoveryData {
    pub enabled: bool,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    #[serde(rename = "endPoint")]
    pub endpoint: String,
    pub provider: Option<String>,
    #[serde(rename = "resourceTypes")]
    pub resource_types: Vec<DiscoveryResourceTypes>,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DiscoveryResourceTypes {
    pub name: String,
    #[serde(rename = "shareTypes")]
    pub share_types: Vec<String>,
    pub protocols: DiscoveryProtocols,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DiscoveryProtocols {
    pub webdav: String,
    pub webapp: Option<String>,
    pub datatx: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShareRequestData {
    #[serde(rename = "ShareWith")]
    pub share_with: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "providerId")]
    pub provider_id: String,
    pub owner: String,
    pub sender: String,
    #[serde(rename = "ownerDisplayName")]
    pub owner_display_name: Option<String>,
    #[serde(rename = "senderDisplayName")]
    pub sender_display_name: Option<String>,
    #[serde(rename = "shareType")]
    pub share_type: String,
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub expiration: u64,
    pub protocol: HashMap<String, ShareRequestProtocol>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShareRequestProtocol {
    pub name: String,
    pub webdav: Option<ShareProtocolWebdav>,
    pub webapp: Option<ShareProtocolWebapp>,
    pub datatx: Option<ShareProtocolDatatx>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShareProtocolWebdav {
    #[serde(rename = "sharedSecret")]
    pub shared_secret: Option<String>,
    pub permissions: Option<Vec<String>>,
    pub uri: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShareProtocolWebapp {
    #[serde(rename = "uriTemplate")]
    pub uri_template: String,
    #[serde(rename = "viewMode")]
    pub view_mode: String,
    #[serde(rename = "sharedSecret")]
    pub shared_secret: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShareProtocolDatatx {
    #[serde(rename = "sharedSecret")]
    pub shared_secret: Option<String>,
    #[serde(rename = "srcUri")]
    pub src_uri: String,
    pub size: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct InviteAcceptedRequestData {
    #[serde(rename = "recipientProvider")]
    pub recipient_provider: String,
    pub token: String,
    #[serde(rename = "userID")]
    pub user_id: String,
    pub email: String,
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct InviteAcceptedResponseData {
    #[serde(rename = "userID")]
    pub user_id: String,
    pub email: String,
    pub name: String,
}
