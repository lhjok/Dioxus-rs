#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Login(cx: Scope) -> Element {
    cx.render(rsx!{
        div {
            id: "login",
            div {
                class: "m-8",
                button {
                    id: "dropdownDefaultButton",
                    "data-dropdown-toggle": "dropdown",
                    class: "text-white bg-blue-700 hover:bg-blue-800 \
                            focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg \
                            text-sm px-5 py-2.5 text-center inline-flex items-center \
                            dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800",
                    r#type: "button",
                    "你要去哪里",
                    svg {
                        class: "w-2.5 h-2.5 ml-2.5",
                        "aria-hidden": "true",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 10 6",
                        path {
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "m1 1 4 4 4-4"
                        }
                    }
                }
                div {
                    id: "dropdown",
                    class: "z-10 hidden bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700",
                    ul {
                        class: "py-2 text-sm text-gray-700 dark:text-gray-200",
                        aria_labelledby: "dropdownDefaultButton",
                        li { Link { to: "/", "Go Home!" } }
                        li { Link { to: "/admin", "Go Admin!" } }
                    }
                }
            }
        }
        script {
            r#type: "module",
            src: "https://cdnjs.cloudflare.com/ajax/libs/flowbite/1.7.0/flowbite.min.js"
        }
    })
}
