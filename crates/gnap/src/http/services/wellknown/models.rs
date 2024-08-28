use serde::{self, Deserialize, Serialize};

// this follows openid-connect-discovery-1_0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenIDConfiguration {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
    pub jwks_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_types_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_modes_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_types_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acr_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_types_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_signing_alg_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_encryption_alg_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_encryption_enc_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userinfo_signing_alg_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userinfo_encryption_alg_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userinfo_encryption_enc_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_object_signing_alg_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_object_encryption_alg_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_object_encryption_enc_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint_auth_methods_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint_auth_signing_alg_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_types_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_documentation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims_locales_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_locales_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims_parameter_supported: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameter_supported: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_uri_parameter_supported: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_request_uri_registration: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_policy_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_tos_uri: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GnapOptions {
    pub service_endpoints: GnapServiceEndpoints,
    pub token_formats_supported: Vec<String>,
    pub interaction_start_modes_supported: Option<Vec<String>>,

    /// a list of the AS's interaction finish methods. The values of this
    /// list correspond to the possible values for the method element of
    /// the interaction finish section (Section 2.5.2) of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_finish_methods_supported: Option<Vec<String>>,

    /// a list of the AS's supported key proofing mechanisms. The values of
    /// this list correspond to possible values of the "proof" field of the key
    /// section (Section 7.1) of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_proofs_supported: Option<Vec<String>>,

    /// a list of the AS's supported subject identifier types.  The values
    /// of this list correspond to possible values of the subject identifier
    /// section (Section 2.2) of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_formats_supported: Option<Vec<String>>,

    /// a list of the AS's supported assertion formats.  The values of this
    /// list correspond to possible values of the subject assertion section
    /// (Section 2.2) of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertions_supported: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GnapServiceEndpoints {
    pub grant_request_endpoint: String,
    pub introspection_endpoint: String,
    pub resource_registration_endpoint: String,
}
