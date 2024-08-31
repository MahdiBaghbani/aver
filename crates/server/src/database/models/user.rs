use native_db::*;
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};

pub mod v1 {
    use super::*;
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    #[native_model(id = 1, version = 1)]
    #[native_db]
    pub struct UserInternal {
        #[primary_key]
        pub id: String,
        #[secondary_key]
        pub username: String,
        pub password: String,
        pub first_name: String,
        pub last_name: String,
        pub email: Option<String>,
        pub created_at: String,
        pub updated_at: String,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    #[native_model(id = 2, version = 1)]
    #[native_db]
    pub struct UserFederated {
        #[primary_key]
        pub id: String,
        pub remote_id: String,
        #[secondary_key]
        pub provider_domain: String,
        #[secondary_key]
        pub username: String,
        pub name: String,
        pub email: String,
        pub created_at: String,
        pub updated_at: String,
    }
}
