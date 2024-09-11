use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::api::user_api::api_login_user;
use crate::components::form_input::FormInput;
use crate::components::submit_button::SubmitButton;
use crate::router::Route;
use crate::store::{set_page_loading, set_show_alert, Store};

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
struct LoginUserSchema {
    #[validate(
        length(min = 1, message = "Username is required"),
    )]
    username: String,
    #[validate(
        length(min = 1, message = "Password is required"),
    )]
    password: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<LoginUserSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data: LoginUserSchema = cloned_form.deref().clone();
        match name {
            "username" => data.username = value,
            "password" => data.password = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let form: UseStateHandle<LoginUserSchema> = use_state(LoginUserSchema::default);
    let validation_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>> = use_state(
        || Rc::new(
            RefCell::new(
                ValidationErrors::new()
            )
        )
    );
    let navigator: Navigator = use_navigator().unwrap();

    let username_input_ref: NodeRef = NodeRef::default();
    let password_input_ref: NodeRef = NodeRef::default();

    let validate_input_on_blur: Callback<(String, String)> = {
        let cloned_form: UseStateHandle<LoginUserSchema> = form.clone();
        let cloned_validation_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>> =
            validation_errors.clone();

        Callback::from(move |(name, value): (String, String)| {
            let mut data: LoginUserSchema = cloned_form.deref().clone();
            match name.as_str() {
                "username" => data.username = value,
                "password" => data.password = value,
                _ => (),
            }
            cloned_form.set(data);

            match cloned_form.validate() {
                Ok(_) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .remove(name.as_str());
                }
                Err(errors) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .retain(|key, _| key != &name);
                    for (field_name, error) in errors.errors() {
                        if field_name == &name {
                            cloned_validation_errors
                                .borrow_mut()
                                .errors_mut()
                                .insert(field_name.clone(), error.clone());
                        }
                    }
                }
            }
        })
    };

    let handle_username_input: Callback<String> = get_input_callback("username", form.clone());
    let handle_password_input: Callback<String> = get_input_callback("password", form.clone());

    let on_submit: Callback<SubmitEvent> = {
        let cloned_form: UseStateHandle<LoginUserSchema> = form.clone();
        let cloned_validation_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>> =
            validation_errors.clone();
        let store_dispatch: Dispatch<Store> = dispatch.clone();
        let cloned_navigator: Navigator = navigator.clone();

        let cloned_username_input_ref: NodeRef = username_input_ref.clone();
        let cloned_password_input_ref: NodeRef = password_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let dispatch: Dispatch<Store> = store_dispatch.clone();
            let form: UseStateHandle<LoginUserSchema> = cloned_form.clone();
            let validation_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>> =
                cloned_validation_errors.clone();
            let navigator: Navigator = cloned_navigator.clone();

            let username_input_ref: NodeRef = cloned_username_input_ref.clone();
            let password_input_ref: NodeRef = cloned_password_input_ref.clone();

            spawn_local(async move {
                match form.validate() {
                    Ok(_) => {
                        let form_data: LoginUserSchema = form.deref().clone();
                        set_page_loading(true, dispatch.clone());

                        let email_input: HtmlInputElement = username_input_ref
                            .cast::<HtmlInputElement>().unwrap();
                        let password_input: HtmlInputElement = password_input_ref
                            .cast::<HtmlInputElement>().unwrap();

                        email_input.set_value("");
                        password_input.set_value("");

                        let form_json: String = serde_json::to_string(&form_data).unwrap();
                        let response: Result<(), String> = api_login_user(&form_json).await;

                        match response {
                            Ok(_) => {
                                set_page_loading(false, dispatch);
                                navigator.push(&Route::Profile);
                            }
                            Err(e) => {
                                set_page_loading(false, dispatch.clone());
                                set_show_alert(e.to_string(), dispatch);
                            }
                        };
                    }
                    Err(e) => {
                        validation_errors.set(Rc::new(RefCell::new(e)));
                    }
                }
            });
        })
    };

    html! {
        <>
            <section class="hero bg-base-300 flex-grow">
                <div class="w-[22rem] flex flex-col items-center justify-center mx-auto lg:py-0">
                    <div class="flex items-center mb-6">
                        <img class="w-28 h-28 mr-2" src="/assets/images/logos/aver.svg" alt="logo"/>
                    </div>
                    <div class="card w-full bg-neutral text-neutral-content shadow-xl">
                        <form class="card-body" onsubmit={on_submit}>
                            <h1 class="card-title">
                                {"Log in to Aver"}
                            </h1>
                            <div class="form-control">
                                <FormInput
                                    label="Username"
                                    name="username"
                                    placeholder=""
                                    input_type="text"
                                    input_ref={username_input_ref}
                                    handle_onchange={handle_username_input}
                                    errors={&*validation_errors}
                                    handle_on_input_blur={validate_input_on_blur.clone()}
                                />
                            </div>
                            <div class="form-control">
                                <FormInput
                                    label="Password"
                                    name="password"
                                    placeholder=""
                                    input_type="password"
                                    input_ref={password_input_ref}
                                    handle_onchange={handle_password_input}
                                    errors={&*validation_errors}
                                    handle_on_input_blur={validate_input_on_blur.clone()}
                                />
                                <label class="label">
                                    <a href="#" class="label-text-alt link link-hover">
                                        {"Forgot password?"}
                                    </a>
                                </label>
                            </div>
                            <div class="form-control mt-6">
                                <SubmitButton
                                    loading={store.page_loading}
                                >
                                    {"Log in"}
                                </SubmitButton>
                            </div>
                            <p class="text-xs label">
                                {"Don’t have an account yet? "}
                                <Link<Route>to={Route::Register} classes="text-accent">
                                    { "Sign up" }
                                </Link<Route>>
                            </p>
                        </form>
                    </div>
                </div>
            </section>
            <footer class="footer footer-center bg-neutral text-base-content p-4">
                <aside>
                    <p>
                        <strong>
                            {"Aver"}
                        </strong>
                        {" - An absolutely unsafe home for all your data, use at your own peril"}
                    </p>
                </aside>
            </footer>
        </>
    }
}
