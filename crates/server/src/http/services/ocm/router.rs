use poem::{get, post, Endpoint, Route};

use super::endpoints::{create_invite_token, invite_accepted, mfa_capable};

pub fn ocm() -> impl Endpoint {
    Route::new()
        .at("/mfa-capable", get(mfa_capable))
        .at("/create-invite-token", post(create_invite_token))
        .at("/invite-accepted", post(invite_accepted))
}
