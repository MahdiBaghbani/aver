use yew::prelude::*;

#[function_component(NavbarNotifications)]
pub fn notification_component() -> Html {
    html! {
        <div
            id="notifications-button"
            class="dropdown dropdown-end"
        >
            <span class="sr-only">{"view notifications"}</span>
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
                        d="M5.25 9a6.75 6.75 0 0 1 13.5 0v.75c0 2.123.8 4.057 2.118 5.52a.75.75 0 0 1-.297 1.206c-1.544.57-3.16.99-4.831 1.243a3.75 3.75 0 1 1-7.48 0 24.585 24.585 0 0 1-4.831-1.244.75.75 0 0 1-.298-1.205A8.217 8.217 0 0 0 5.25 9.75V9Zm4.502 8.9a2.25 2.25 0 1 0 4.496 0 25.057 25.057 0 0 1-4.496 0Z"
                    />
                </svg>
            </div>
            <div
                id="notifications-dropdown"
                class="dropdown-content z-[1] my-4 w-64 text-base list-none bg-neutral \
                    rounded divide-y divide-gray-100 shadow dark:divide-gray-600"
            >
                <div
                    class="block bg-secondary py-2 px-4 text-base font-medium text-center \
                        text-gray-700 dark:text-gray-300"
                >
                    {"Notifications"}
                </div>
                <div>
                    <a
                        href="#"
                        class="flex py-3 px-4 border-b hover:bg-gray-100 dark:hover:bg-gray-600 dark:border-gray-600"
                    >
                        <div class="flex-shrink-0">
                            <img
                                class="w-11 h-11 rounded-full"
                                src="https://flowbite.s3.amazonaws.com/blocks/marketing-ui/avatars/bonnie-green.png"
                                alt="Bonnie Green avatar"
                            />
                            <div
                                class="flex absolute justify-center items-center ml-6 -mt-5 w-5 h-5 rounded-full border border-white bg-primary-700 dark:border-gray-700"
                            >
                                <svg
                                    aria-hidden="true"
                                    class="w-3 h-3 text-white"
                                    fill="currentColor"
                                    viewBox="0 0 20 20"
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <path
                                        d="M8.707 7.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l2-2a1 1 0 00-1.414-1.414L11 7.586V3a1 1 0 10-2 0v4.586l-.293-.293z"
                                    ></path>
                                    <path
                                        d="M3 5a2 2 0 012-2h1a1 1 0 010 2H5v7h2l1 2h4l1-2h2V5h-1a1 1 0 110-2h1a2 2 0 012 2v10a2 2 0 01-2 2H5a2 2 0 01-2-2V5z"
                                    ></path>
                                </svg>
                            </div>
                        </div>
                        <div class="pl-3 w-full">
                            <div
                                class="text-gray-500 font-normal text-sm mb-1.5 dark:text-gray-400"
                            >
                                {"This is a notification"}
                            </div>
                        </div>
                    </a>
                </div>
                <a
                    href="#"
                    class="block py-2 text-md font-medium text-center text-gray-900 bg-gray-50 hover:bg-gray-100 dark:bg-gray-600 dark:text-white dark:hover:underline"
                >
                    <div class="inline-flex items-center">
                        <svg
                            aria-hidden="true"
                            class="mr-2 w-4 h-4 text-gray-500 dark:text-gray-400"
                            fill="currentColor"
                            viewBox="0 0 20 20"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path d="M10 12a2 2 0 100-4 2 2 0 000 4z"></path>
                            <path
                                fill-rule="evenodd"
                                d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z"
                                clip-rule="evenodd"
                            >
                            </path>
                        </svg>
                        {"View all"}
                    </div>
                </a>
            </div>
        </div>
    }
}