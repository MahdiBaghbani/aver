use poem::web::Json;
use poem::{handler, IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum HealthCheckStatus {
    Pass,
}

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub status: HealthCheckStatus,
}

#[handler]
pub async fn health() -> impl IntoResponse {
    Json(HealthCheckResponse {
        status: HealthCheckStatus::Pass,
    })
}
