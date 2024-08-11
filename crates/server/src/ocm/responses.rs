use poem_openapi::payload::Json;
use poem_openapi::ApiResponse;

use crate::ocm::models::DiscoveryData;

#[derive(ApiResponse)]
pub enum DiscoveryResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<DiscoveryData>),
}