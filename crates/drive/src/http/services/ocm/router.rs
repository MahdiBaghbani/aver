use poem::{get, post, Endpoint, Route};

use crate::http::EndpointMode;

use super::endpoints::{create_invite_token_ui, create_invite_token_return_html, create_invite_token_return_json, invite_accepted, mfa_capable};

pub fn ocm(mode: EndpointMode) -> impl Endpoint {
    match mode {
        EndpointMode::Api => {
            Route::new()
                .at("/create-invite-token", get(create_invite_token_return_json))
        }
        EndpointMode::Ssr => {
            Route::new()
                .at("/mfa-capable", get(mfa_capable))
                .at(
                    "/create-invite-token",
                    get(create_invite_token_ui).post(create_invite_token_return_html),
                )
                .at("/invite-accepted", post(invite_accepted))
        }
    }
}
