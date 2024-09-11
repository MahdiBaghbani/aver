use yew::prelude::*;

#[function_component(NavbarUserMenu)]
pub fn user_menu_component() -> Html {
    html! {
        <div
            id="user-menu-button"
            class="dropdown dropdown-end"
        >
            <span class="sr-only">{"open user menu"}</span>
            <div tabindex="0" role="button" class="btn btn-ghost btn-circle avatar">
                <div class="w-10 h-10 rounded-full">
                    <img
                        alt="user profile picture"
                        src="https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp"
                    />
                </div>
            </div>
            <div
                id="user-menu-dropdown"
                class="dropdown-content z-[1] my-4 w-64 text-base list-none bg-neutral \
                rounded divide-y divide-gray-100 shadow dark:divide-gray-600"
            >
                <div class="py-3 px-4">
                    <span
                        class="block text-sm font-semibold text-gray-900 dark:text-white"
                    >
                        {"Mahdi Baghbani"}
                    </span>
                    <span
                        class="block text-sm text-gray-900 truncate dark:text-white"
                    >
                        {"mahdi-baghbani@aver.dev"}
                    </span>
                </div>
                <ul
                    class="py-1 text-gray-700 dark:text-gray-300"
                    aria-labelledby="user-menu-dropdown"
                >
                    <li>
                        <a
                            href="#"
                            class="flex items-center py-2 px-4 text-sm hover:bg-gray-100 \
                            dark:hover:bg-gray-600 dark:hover:text-white"
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                class="mr-2 w-5 h-5 text-gray-400"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M15.75 6a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0ZM4.501 20.118a7.5 7.5 0 0 1 14.998 0A17.933 17.933 0 0 1 12 21.75c-2.676 0-5.216-.584-7.499-1.632Z"
                                    />
                            </svg>
                            {"Profile"}
                        </a>
                    </li>
                    <li>
                        <a
                            href="#"
                            class="flex items-center py-2 px-4 text-sm hover:bg-gray-100 \
                            dark:hover:bg-gray-600 dark:hover:text-white"
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                class="mr-2 w-5 h-5 text-gray-400"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.955.26 1.43l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-2.594c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.431l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.247a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.086.22-.128.332-.183.582-.495.644-.869l.214-1.28Z"
                                />
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
                                />
                            </svg>
                            {"Preferences"}
                        </a>
                    </li>
                </ul>
                <ul
                    class="py-1 text-gray-700 dark:text-gray-300"
                    aria-labelledby="dropdown"
                >
                    <li>
                        <a
                            href="#"
                            class="flex items-center py-2 px-4 text-sm hover:bg-gray-100 \
                            dark:hover:bg-gray-600 dark:hover:text-white"
                        >   
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                class="mr-2 w-5 h-5 text-gray-400"
                            >
                                <path 
                                    stroke-linecap="round" 
                                    stroke-linejoin="round" 
                                    d="M15.75 9V5.25A2.25 2.25 0 0 0 13.5 3h-6a2.25 2.25 0 0 0-2.25 2.25v13.5A2.25 2.25 0 0 0 7.5 21h6a2.25 2.25 0 0 0 2.25-2.25V15m3 0 3-3m0 0-3-3m3 3H9"
                                />
                            </svg>
                            {"Log out"}
                        </a>
                    </li>
                </ul>
            </div>
        </div>
    }
}
