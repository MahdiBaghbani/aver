use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LoginRequestData {
    pub username: String,
    pub password: String,
}
