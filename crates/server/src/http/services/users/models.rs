use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterUserRequestData {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RegisterUserResponseData {
    pub id: Uuid,
    pub firstname: String,
    pub lastname: String,
    pub username: String,
}

