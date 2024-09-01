use poem::{post, Endpoint, Route};

use super::endpoints::create;

pub fn users() -> impl Endpoint {
    Route::new()
        .at("/create", post(create))
}
