use poem::{get, Endpoint, Route};

use crate::http::EndpointType;

use super::endpoints::{login_form, login_ui, logout};

pub fn auth(mode: EndpointType) -> impl Endpoint {
    match mode {
        EndpointType::Api => {
            Route::new()
                .at("/login", get(login_ui).post(login_form))
        }
        EndpointType::Ssr => {
            Route::new()
                .at("/login", get(login_ui).post(login_form))
                .at("/logout", get(logout))
        }
    }
}
