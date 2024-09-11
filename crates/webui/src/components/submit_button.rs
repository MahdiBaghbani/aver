use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub loading: bool,
    pub children: Children,
}

#[function_component(SubmitButton)]
pub fn submit_button_component(props: &Props) -> Html {
    html! {
    <button
      type="submit"
      class={format!(
        "btn btn-active {}",
         if props.loading {
            "btn-secondary"
         } else {
            "btn-accent"
         }
      )}
    >
      if props.loading {
            <span class="loading loading-spinner"></span>
            {"Loading"}
      }else{
        {props.children.clone()}
      }
    </button>
    }
}
