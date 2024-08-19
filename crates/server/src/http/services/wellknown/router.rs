use poem::{get, Endpoint, Route};

use crate::http::services::ocm::endpoints::discovery;

pub fn wellknown() -> impl Endpoint {
    Route::new()
        .at("/ocm", get(discovery))
}