use poem::{get, post, Route};

use crate::ocm::endpoints::{invite_accepted, mfa_capable};

pub fn ocm() -> Route {
    Route::new()
        .at("/mfa-capable", get(mfa_capable))
        .at("/invite-accepted", post(invite_accepted))
}
