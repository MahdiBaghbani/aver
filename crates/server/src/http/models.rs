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

#[derive(Clone, Debug)]
pub struct PanicHandler {}
