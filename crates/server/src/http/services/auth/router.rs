use poem::{get, Endpoint, Route};

use super::endpoints::{login_form, login_ui};

pub fn auth() -> impl Endpoint {
    Route::new()
        .at("/login", get(login_ui).post(login_form))
}
