use yew::prelude::*;

#[function_component(Footer)]
pub fn footer_component() -> Html {
    html! {
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
    }
}
