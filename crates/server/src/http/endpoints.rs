use poem::web::Json;
use poem::{handler, IntoResponse};

use crate::http::models::{HealthCheckResponse, HealthCheckStatus};

#[handler]
pub async fn health() -> impl IntoResponse {
    Json(HealthCheckResponse {
        status: HealthCheckStatus::Pass,
    })
}
