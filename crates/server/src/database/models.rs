use native_db::*;
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[native_model(id = 1, version = 1)]
#[native_db]
pub struct User {
    #[primary_key]
    pub id: String,
    #[secondary_key]
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
