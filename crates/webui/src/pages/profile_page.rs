use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

use crate::components::navbar::Navbar;
use crate::store::Store;

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let user = store.auth_user.clone();
    let navigator = use_navigator().unwrap();

    // use_effect_with_deps(
    //     move |_| {
    //         let dispatch = dispatch.clone();
    //         wasm_bindgen_futures::spawn_local(async move {
    //             set_page_loading(true, dispatch.clone());
    //             let response = api_user_info().await;
    //             match response {
    //                 Ok(user) => {
    //                     set_page_loading(false, dispatch.clone());
    //                     set_auth_user(Some(user), dispatch);
    //                 }
    //                 Err(e) => {
    //                     set_page_loading(false, dispatch.clone());
    //                     set_show_alert(e.to_string(), dispatch);
    //                     navigator.push(&router::Route::Login);
    //                 }
    //             }
    //         });
    //     },
    //     (),
    // );

    html! {
        <section class="antialiased bg-base">
            <Navbar/>
         </section>
    }
}
