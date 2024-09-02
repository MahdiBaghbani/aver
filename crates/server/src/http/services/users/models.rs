use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CreateUserRequestData {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub password: String,
}
