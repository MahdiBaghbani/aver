use poem::{get, post, Endpoint, Route};

use crate::http::EndpointType;

use super::endpoints::{register_ui, register_with_form, register_with_json};

pub fn users(mode: EndpointType) -> impl Endpoint {
    match mode {
        EndpointType::Api => {
            Route::new()
                .at("/register", post(register_with_json))
        }
        EndpointType::Ssr => {
            Route::new()
                .at("/register", get(register_ui).post(register_with_form))
        }
    }
}
