use serde::{Deserialize, Serialize};

pub enum LoginMode {
    Cookie,
    Token,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LoginRequestData {
    pub username: String,
    pub password: String,
}
