use poem::{get, Endpoint, Route};

use crate::http::services::ocm::endpoints::legacy_discovery;
use crate::utils::health::health;

use super::endpoints::index;

pub fn root() -> impl Endpoint {
    Route::new()
        .at("/", get(index))
        .at("/ocm-provider", get(legacy_discovery))
        .at("/health", get(health))
}
