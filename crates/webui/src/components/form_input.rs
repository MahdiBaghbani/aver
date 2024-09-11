use std::cell::Ref;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
use validator::{ValidationError, ValidationErrors};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or(AttrValue::from("text"))]
    pub input_type: AttrValue,
    #[prop_or_default]
    pub placeholder: AttrValue,
    pub label: AttrValue,
    pub name: AttrValue,
    pub input_ref: NodeRef,
    pub handle_onchange: Callback<String>,
    pub handle_on_input_blur: Callback<(String, String)>,
    pub errors: Rc<RefCell<ValidationErrors>>,
}

#[function_component(FormInput)]
pub fn form_input_component(props: &Props) -> Html {
    let val_errors: Ref<ValidationErrors> = props.errors.borrow();
    let errors: HashMap<&str, &Vec<ValidationError>> = val_errors.field_errors().clone();
    let empty_errors: Vec<ValidationError> = vec![];

    let error: &Vec<ValidationError> = match errors.get(&props.name.as_str()) {
        Some(error) => error,
        None => &empty_errors,
    };


    let handle_onchange: Callback<String> = props.handle_onchange.clone();
    let onchange: Callback<Event> = Callback::from(move |event: Event| {
        let target: EventTarget = event.target().unwrap();
        let value: String = target.unchecked_into::<HtmlInputElement>().value();
        handle_onchange.emit(value);
    });

    let handle_on_input_blur: Callback<(String, String)> = props.handle_on_input_blur.clone();
    let on_blur: Callback<FocusEvent> = {
        let cloned_input_name: AttrValue = props.name.clone();
        Callback::from(move |event: FocusEvent| {
            let input_name: AttrValue = cloned_input_name.clone();
            let target: EventTarget = event.target().unwrap();
            let value: String = target.unchecked_into::<HtmlInputElement>().value();
            handle_on_input_blur.emit((input_name.parse().unwrap(), value));
        })
    };

    html! {
        <>
            <label class="label" for={props.name.clone()}>
                <span class="label-text">{props.label.clone()}</span>
            </label>
            <input
                id={props.name.clone()}
                type={props.input_type.clone()}
                placeholder={props.placeholder.clone()}
                class={format!(
                    "input input-bordered input-md w-full max-w-xs {}",
                     if !error.is_empty() {
                        "input-error"
                     } else {
                        ""
                     }
                )}
                ref={props.input_ref.clone()}
                onchange={onchange}
                onblur={on_blur}
            />
        </>
    }
}
