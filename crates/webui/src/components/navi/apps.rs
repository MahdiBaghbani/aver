use yew::prelude::*;

#[function_component(NavbarAppsMenu)]
pub fn apps_menu_component() -> Html {
    html! {
        <div
            id="apps-menu-button"
            class="dropdown dropdown-end"
        >
            <span class="sr-only">{"open apps menu"}</span>
            <div tabindex="0" role="button" class="btn btn-ghost btn-circle">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    class="h-6 w-6"
                >
                    <path
                        fill-rule="evenodd"
                        clip-rule="evenodd"
                        d="M3 6a3 3 0 0 1 3-3h2.25a3 3 0 0 1 3 3v2.25a3 3 0 0 1-3 3H6a3 3 0 0 1-3-3V6Zm9.75 0a3 3 0 0 1 3-3H18a3 3 0 0 1 3 3v2.25a3 3 0 0 1-3 3h-2.25a3 3 0 0 1-3-3V6ZM3 15.75a3 3 0 0 1 3-3h2.25a3 3 0 0 1 3 3V18a3 3 0 0 1-3 3H6a3 3 0 0 1-3-3v-2.25Zm9.75 0a3 3 0 0 1 3-3H18a3 3 0 0 1 3 3V18a3 3 0 0 1-3 3h-2.25a3 3 0 0 1-3-3v-2.25Z"
                    />
                </svg>
            </div>
            <div
                id="apps-menu-dropdown"
                class="dropdown-content z-[1] my-4 w-64 text-base list-none bg-neutral \
                rounded divide-y divide-gray-100 shadow dark:divide-gray-600"
            >
                <div
                    class="block bg-secondary py-2 px-4 text-base font-medium text-center text-gray-700"
                >
                    {"Apps"}
                </div>
                <div class="grid grid-cols-3 gap-2 p-2">
                    <a
                        href="#"
                        class="block p-4 text-center rounded-lg hover:bg-gray-100 dark:hover:bg-gray-600 group"
                    >
                        <svg
                                aria-hidden="true"
                                class="mx-auto mb-1 w-7 h-7 text-gray-400 group-hover:text-gray-500 dark:text-gray-400 dark:group-hover:text-gray-400"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                                xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1"
                            ></path>
                        </svg>
                        <div class="text-sm text-gray-900 dark:text-white">
                            {"Logout"}
                        </div>
                    </a>
                </div>
            </div>
        </div>
    }
}
