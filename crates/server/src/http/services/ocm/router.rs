use poem::{get, post, Endpoint, Route};

use crate::http::services::ocm::endpoints::{invite_accepted, mfa_capable};

pub fn ocm() -> impl Endpoint {
    Route::new()
        .at("/mfa-capable", get(mfa_capable))
        .at("/invite-accepted", post(invite_accepted))
}
