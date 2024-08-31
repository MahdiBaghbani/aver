use native_db::{Builder, Database, Models};
use once_cell::sync::{Lazy, OnceCell};

use crate::settings::settings;

pub mod models;

pub static DATABASE_MODELS: Lazy<Models> = Lazy::new(|| {
    let mut models: Models = Models::new();
    models
        .define::<models::user::v1::User>()
        .expect("failed to define model User");
    models
});

pub static DATABASE: OnceCell<Database> = OnceCell::new();

pub fn database() -> &'static Database<'static> {
    DATABASE.get().expect("database init")
}

pub fn init() {
    let database: Database = Builder::new().create(
        &DATABASE_MODELS,
        settings().database.get_db_file_path(),
    ).unwrap();
    DATABASE.set(database).ok();
}
