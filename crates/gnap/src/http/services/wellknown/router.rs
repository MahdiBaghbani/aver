use poem::{get, Endpoint, Route};

use crate::http::services::wellknown::endpoints::openid_config;

pub fn wellknown() -> impl Endpoint {
    Route::new()
        .at("/openid-configuration", get(openid_config))
}