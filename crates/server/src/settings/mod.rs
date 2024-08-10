use once_cell::sync::OnceCell;

pub mod methods;
pub mod models;

use crate::settings::models::Settings;

pub static SETTINGS: OnceCell<Settings> = OnceCell::new();
