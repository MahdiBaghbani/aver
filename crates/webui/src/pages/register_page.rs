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

use crate::api::user_api::api_register_user;
use crate::components::{form_input::FormInput, submit_button::SubmitButton};
use crate::router::Route;
use crate::store::{set_page_loading, set_show_alert, Store};

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
struct RegisterUserSchema {
    #[validate(length(min = 1, message = "Firstname is required"))]
    firstname: String,
    #[validate(length(min = 1, message = "Lastname is required"))]
    lastname: String,
    #[validate(
        length(min = 1, message = "Username is required"),
    )]
    username: String,
    #[validate(length(min = 1, message = "Password is required"))]
    password: String,
    #[validate(
        length(min = 1, message = "Please confirm your password"),
        must_match(other = "password", message = "Passwords do not match")
    )]
    password_confirm: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<RegisterUserSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "firstname" => data.firstname = value,
            "lastname" => data.lastname = value,
            "username" => data.username = value,
            "password" => data.password = value,
            "password_confirm" => data.password_confirm = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

#[function_component(RegisterPage)]
pub fn register_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let form: UseStateHandle<RegisterUserSchema> = use_state(|| RegisterUserSchema::default());
    let validation_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>> = use_state(
        || Rc::new(
            RefCell::new(
                ValidationErrors::new()
            )
        )
    );
    let navigator: Navigator = use_navigator().unwrap();

    let firstname_input_ref: NodeRef = NodeRef::default();
    let lastname_input_ref: NodeRef = NodeRef::default();
    let username_input_ref: NodeRef = NodeRef::default();
    let password_input_ref: NodeRef = NodeRef::default();
    let password_confirm_input_ref: NodeRef = NodeRef::default();

    let validate_input_on_blur: Callback<(String, String)> = {
        let cloned_form: UseStateHandle<RegisterUserSchema> = form.clone();
        let cloned_validation_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>> =
            validation_errors.clone();

        Callback::from(move |(name, value): (String, String)| {
            let mut data: RegisterUserSchema = cloned_form.deref().clone();

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

    let handle_firstname_input: Callback<String> = get_input_callback(
        "firstname",
        form.clone(),
    );
    let handle_lastname_input: Callback<String> = get_input_callback(
        "lastname",
        form.clone(),
    );
    let handle_username_input: Callback<String> = get_input_callback(
        "username",
        form.clone(),
    );
    let handle_password_input: Callback<String> = get_input_callback(
        "password",
        form.clone(),
    );
    let handle_password_confirm_input: Callback<String> = get_input_callback(
        "password_confirm",
        form.clone(),
    );

    let on_submit: Callback<SubmitEvent> = {
        let cloned_form: UseStateHandle<RegisterUserSchema> = form.clone();
        let cloned_validation_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>> =
            validation_errors.clone();
        let cloned_navigator: Navigator = navigator.clone();
        let cloned_dispatch: Dispatch<Store> = dispatch.clone();

        let cloned_firstname_input_ref: NodeRef = firstname_input_ref.clone();
        let cloned_lastname_input_ref: NodeRef = lastname_input_ref.clone();
        let cloned_username_input_ref: NodeRef = username_input_ref.clone();
        let cloned_password_input_ref: NodeRef = password_input_ref.clone();
        let cloned_password_confirm_input_ref: NodeRef = password_confirm_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            let form: UseStateHandle<RegisterUserSchema> = cloned_form.clone();
            let validation_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>> = cloned_validation_errors.clone();
            let navigator: Navigator = cloned_navigator.clone();
            let dispatch: Dispatch<Store> = cloned_dispatch.clone();

            let firstname_input_ref: NodeRef = cloned_firstname_input_ref.clone();
            let lastname_input_ref: NodeRef = cloned_lastname_input_ref.clone();
            let username_input_ref: NodeRef = cloned_username_input_ref.clone();
            let password_input_ref: NodeRef = cloned_password_input_ref.clone();
            let password_confirm_input_ref: NodeRef = cloned_password_confirm_input_ref.clone();

            event.prevent_default();
            spawn_local(async move {
                match form.validate() {
                    Ok(_) => {
                        let form_data = form.deref().clone();
                        let form_json = serde_json::to_string(&form_data).unwrap();
                        set_page_loading(true, dispatch.clone());

                        let firstname_input = firstname_input_ref.cast::<HtmlInputElement>().unwrap();
                        let lastname_input = lastname_input_ref.cast::<HtmlInputElement>().unwrap();
                        let username_input = username_input_ref.cast::<HtmlInputElement>().unwrap();
                        let password_input = password_input_ref.cast::<HtmlInputElement>().unwrap();
                        let password_confirm_input = password_confirm_input_ref
                            .cast::<HtmlInputElement>()
                            .unwrap();

                        firstname_input.set_value("");
                        lastname_input.set_value("");
                        username_input.set_value("");
                        password_input.set_value("");
                        password_confirm_input.set_value("");

                        let res = api_register_user(&form_json).await;
                        match res {
                            Ok(_) => {
                                set_page_loading(false, dispatch.clone());
                                set_show_alert(
                                    "Account registered successfully".to_string(),
                                    dispatch,
                                );
                                navigator.push(&Route::Login);
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
    <section class="py-8 bg-ct-blue-600 min-h-screen grid place-items-center">
      <div class="w-full">
        <h1 class="text-4xl xl:text-6xl text-center font-[600] text-ct-yellow-600 mb-4">
         {" Welcome to CodevoWeb!"}
        </h1>
        <h2 class="text-lg text-center mb-4 text-ct-dark-200">
          {"Sign Up To Get Started!"}
        </h2>
          <form
            onsubmit={on_submit}
            class="max-w-md w-full mx-auto overflow-hidden shadow-lg bg-ct-dark-200 rounded-2xl p-8 space-y-5"
          >
            <FormInput label="Firstname" name="firstname" input_ref={firstname_input_ref} handle_onchange={handle_firstname_input}  errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
            <FormInput label="Lastname" name="lastname" input_ref={lastname_input_ref} handle_onchange={handle_lastname_input}  errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
            <FormInput label="Username" name="username" input_type="username" input_ref={username_input_ref} handle_onchange={handle_username_input}  errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
            <FormInput label="Password" name="password" input_type="password" input_ref={password_input_ref} handle_onchange={handle_password_input}  errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
            <FormInput
              label="Confirm Password"
              name="password_confirm"
              input_type="password"
              input_ref={password_confirm_input_ref}
              handle_onchange={handle_password_confirm_input}
              errors={&*validation_errors}
              handle_on_input_blur={validate_input_on_blur.clone()}
            />
            <span class="block">
              {"Already have an account?"} {" "}
            <Link<Route> to={Route::Login} classes="text-ct-blue-600">{"Login Here"}</Link<Route>>
            </span>
            <SubmitButton
              loading={store.page_loading}
            >
             {" Sign Up"}
            </SubmitButton>
          </form>
      </div>
    </section>
    }
}
