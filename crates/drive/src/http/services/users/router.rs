use poem::{get, post, Endpoint, Route};

use crate::http::EndpointMode;

use super::endpoints::{me, register_ui, register_with_form, register_with_json};

pub fn users(mode: EndpointMode) -> impl Endpoint {
    match mode {
        EndpointMode::Api => {
            Route::new()
                .at("/me", get(me))
                .at("/register", post(register_with_json))
        }
        EndpointMode::Ssr => {
            Route::new()
                .at("/register", get(register_ui).post(register_with_form))
        }
    }
}
