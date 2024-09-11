use yew::prelude::*;
use yew_router::components::Redirect;
use yewdux::functional::use_store;

use crate::api::types::User;
use crate::components::header::Header;
use crate::router::Route;
use crate::store::Store;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let (store, _dispatch) = use_store::<Store>();

    let _user: User = match store.auth_user.clone() {
        Some(user) => user,
        // redirects to the login page when user is `None`.
        None => return html! {
            <Redirect<Route> to={Route::Login}/>
        },
    };

    html! {
      <>
        <Header />
        <section class="bg-ct-blue-600 min-h-screen pt-20">
            <div class="max-w-4xl mx-auto bg-ct-dark-100 rounded-md h-[20rem] flex justify-center items-center">
                <p class="text-3xl font-semibold">{"Welcome to Rust, Yew.rs and WebAssembly"}</p>
            </div>
        </section>
      </>
    }
}
