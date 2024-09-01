use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CreateUserRequestData {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
}
