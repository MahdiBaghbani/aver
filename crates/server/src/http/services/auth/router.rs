use poem::{get, post, Endpoint, Route};

use crate::http::EndpointType;

use super::endpoints::{login_ui, login_with_form, login_with_json, logout};

pub fn auth(mode: EndpointType) -> impl Endpoint {
    match mode {
        EndpointType::Api => {
            Route::new()
                .at("/login", post(login_with_json))
        }
        EndpointType::Ssr => {
            Route::new()
                .at("/login", get(login_ui).post(login_with_form))
                .at("/logout", get(logout))
        }
    }
}
