use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

pub mod user;
pub mod apps;
pub mod notifications;
pub mod search;

use crate::{
    api::user_api::api_logout_user,
    router::{self},
    store::{set_auth_user, set_page_loading, set_show_alert, Store},
};

use self::apps::NavbarAppsMenu;
use self::notifications::NavbarNotifications;
use self::search::NavbarSearch;
use self::user::NavbarUserMenu;

#[function_component(Navbar)]
pub fn navbar_component() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let _user = store.auth_user.clone();
    let navigator = use_navigator().unwrap();

    let _handle_logout = {
        let store_dispatch = dispatch.clone();
        let cloned_navigator = navigator.clone();

        Callback::from(move |_: MouseEvent| {
            let dispatch = store_dispatch.clone();
            let navigator = cloned_navigator.clone();
            spawn_local(async move {
                set_page_loading(true, dispatch.clone());
                let res = api_logout_user().await;
                match res {
                    Ok(_) => {
                        set_page_loading(false, dispatch.clone());
                        set_auth_user(None, dispatch.clone());
                        set_show_alert("Logged out successfully".to_string(), dispatch);
                        navigator.push(&router::Route::Login);
                    }
                    Err(e) => {
                        set_show_alert(e.to_string(), dispatch.clone());
                        set_page_loading(false, dispatch);
                    }
                };
            });
        })
    };

    html! {
        <nav class="bg-neutral border-b border-gray-200 px-4 py-2.5 dark:border-gray-700 fixed left-0 right-0 top-0 z-50">
            <div class="flex flex-wrap justify-between items-center">
                <div class="flex justify-start items-center">
                    <a href="/profile" class="flex items-center justify-between mr-4">
                        <img class="mr-3 h-10" src="/assets/images/logos/aver-with-text.svg" alt="Aver logo"/>
                    </a>
                </div>
                <div class="flex items-center lg:order-2">
                    <NavbarSearch/>
                </div>
                <div class="flex items-center lg:order-2">
                    <NavbarNotifications/>
                    <NavbarAppsMenu/>
                    <NavbarUserMenu/>
                </div>
            </div>
        </nav>
    }
}
