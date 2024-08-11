use poem::{post, Route};


use crate::ocm::endpoints::invite_accepted;

pub fn ocm() -> Route {
    Route::new()
        .at("/invite-accepted", post(invite_accepted))
}
