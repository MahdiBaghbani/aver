use yew::prelude::*;

#[function_component(NavbarSearch)]
pub fn search_component() -> Html {
    html! {
        <form action="#" method="GET" class="hidden md:block md:pl-2">
            <label for="navbar-search" class="sr-only">{"Search"}</label>
            <div class="relative md:w-64 md:w-96">
                <div class="flex absolute inset-y-0 left-0 items-center pl-3 pointer-events-none">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="currentColor"
                        viewBox="0 0 20 20"
                        class="w-4 h-4 text-gray-500 dark:text-gray-400"
                    >
                        <path
                            fill-rule="evenodd"
                            clip-rule="evenodd"
                            d="M10.5 3.75a6.75 6.75 0 1 0 0 13.5 6.75 6.75 0 0 0 0-13.5ZM2.25 10.5a8.25 8.25 0 1 1 14.59 5.28l4.69 4.69a.75.75 0 1 1-1.06 1.06l-4.69-4.69A8.25 8.25 0 0 1 2.25 10.5Z"
                        />
                    </svg>
                </div>
                <input
                    type="text"
                    name="search"
                    id="navbar-search"
                    class="input input-bordered block w-full max-w-xs text-sm rounded-lg pl-10 p-2.5"
                    placeholder="Search"
                />
            </div>
        </form>
    }
}