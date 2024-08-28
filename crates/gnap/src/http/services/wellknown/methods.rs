use crate::http::services::wellknown::models::{
    GnapOptions,
    GnapServiceEndpoints,
    OpenIDConfiguration,
};

impl OpenIDConfiguration {
    pub fn new(base_url: String) -> Self {
        OpenIDConfiguration {
            issuer: base_url.to_string(),
            authorization_endpoint: format!("{}/gnap/auth", base_url),
            token_endpoint: format!("{}/gnap/token", base_url),
            userinfo_endpoint: format!("{}/gnap/userinfo", base_url),
            jwks_uri: format!("{}/gnap/jwks", base_url),
            registration_endpoint: None,
            scopes_supported: None,
            response_types_supported: None,
            response_modes_supported: None,
            grant_types_supported: None,
            acr_values_supported: None,
            subject_types_supported: None,
            id_token_signing_alg_values_supported: None,
            id_token_encryption_alg_values_supported: None,
            id_token_encryption_enc_values_supported: None,
            userinfo_signing_alg_values_supported: None,
            userinfo_encryption_alg_values_supported: None,
            userinfo_encryption_enc_values_supported: None,
            request_object_signing_alg_values_supported: None,
            request_object_encryption_alg_values_supported: None,
            request_object_encryption_enc_values_supported: None,
            token_endpoint_auth_methods_supported: None,
            token_endpoint_auth_signing_alg_values_supported: None,
            display_values_supported: None,
            claim_types_supported: None,
            claims_supported: None,
            service_documentation: None,
            claims_locales_supported: None,
            ui_locales_supported: None,
            claims_parameter_supported: None,
            request_parameter_supported: None,
            request_uri_parameter_supported: None,
            require_request_uri_registration: None,
            op_policy_uri: None,
            op_tos_uri: None,
        }
    }
}

impl GnapOptions {
    pub fn new(base_url: String) -> Self {
        GnapOptions {
            service_endpoints: GnapServiceEndpoints {
                grant_request_endpoint: format!("{}/gnap/tx", base_url),
                introspection_endpoint: format!("{}/gnap/introspect", base_url),
                resource_registration_endpoint: format!("{}/gnap/resource", base_url),
            },
            token_formats_supported: vec![
                "jwt".to_owned(),
                "paseto".to_owned()
            ],
            interaction_start_modes_supported: None,
            interaction_finish_methods_supported: None,
            key_proofs_supported: None,
            subject_formats_supported: None,
            assertions_supported: None,
        }
    }
}
