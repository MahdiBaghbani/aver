use native_db::Models;
use once_cell::sync::Lazy;

pub mod models;

static DATABASE_MODELS: Lazy<Models> = Lazy::new(|| {
    let mut models = Models::new();
    models
        .define::<models::User>()
        .expect("failed to define model User");
    models
});
