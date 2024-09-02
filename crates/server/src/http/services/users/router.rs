use poem::{get, Endpoint, Route};

use super::endpoints::{create, create_ui};

pub fn users() -> impl Endpoint {
    Route::new()
        .at("/create", get(create_ui).post(create))
}
