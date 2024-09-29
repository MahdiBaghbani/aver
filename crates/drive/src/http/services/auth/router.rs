use poem::{get, post, Endpoint, Route};

use crate::http::EndpointMode;

use super::endpoints::{login_ui, login_with_form, login_with_json, logout};

pub fn auth(mode: EndpointMode) -> impl Endpoint {
    match mode {
        EndpointMode::Api => {
            Route::new()
                .at("/login", post(login_with_json))
        }
        EndpointMode::Ssr => {
            Route::new()
                .at("/login", get(login_ui).post(login_with_form))
                .at("/logout", get(logout))
        }
    }
}
