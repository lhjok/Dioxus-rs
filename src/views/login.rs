#![allow(non_snake_case)]
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use dioxus_router::Link;
use web_sys::console;

#[wasm_bindgen(module=
"/static/scripts/login.js")]
extern {
    fn init_te();
}

#[wasm_bindgen]
pub fn init_elements() {
    init_te()
}

pub fn Login(cx: Scope) -> Element {
    use_effect(cx, (), |()| async move {
        init_elements();
    });

    use_effect(cx, (), |()| async move {
        console::log_1(&"Login".into());
    });

    cx.render(rsx!{
        div {
            id: "login",
            div {
                class: "m-8 relative",
                "data-te-dropdown-ref": "",
                button {
                    r#type: "button",
                    id: "dropdownMenuButton1",
                    class:
                    "flex items-center whitespace-nowrap rounded \
                     bg-primary px-6 pb-2 pt-2.5 text-xs font-medium \
                     uppercase leading-normal text-white shadow-[0_4px_9px_-4px_#3b71ca] \
                     transition duration-150 ease-in-out hover:bg-primary-600 \
                     hover:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.3),0_4px_18px_0_rgba(59,113,202,0.2)] \
                     focus:bg-primary-600 focus:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.3),0_4px_18px_0_rgba(59,113,202,0.2)] \
                     focus:outline-none focus:ring-0 active:bg-primary-700 \
                     active:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.3),0_4px_18px_0_rgba(59,113,202,0.2)] \
                     motion-reduce:transition-none dark:shadow-[0_4px_9px_-4px_rgba(59,113,202,0.5)] \
                     dark:hover:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.2),0_4px_18px_0_rgba(59,113,202,0.1)] \
                     dark:focus:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.2),0_4px_18px_0_rgba(59,113,202,0.1)] \
                     dark:active:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.2),0_4px_18px_0_rgba(59,113,202,0.1)]",
                    "data-te-dropdown-toggle-ref": "",
                    "aria-expanded": "false",
                    "data-te-ripple-init": "",
                    "data-te-ripple-color": "light",
                    "Dropdown button",
                    span {
                        class: "ml-2 w-2",
                        svg {
                            class: "h-5 w-5",
                            fill: "currentColor",
                            "viewBox": "0 0 20 20",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                "fill-rule": "evenodd",
                                d: "M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 \
                                    0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z",
                                "clip-rule": "evenodd"
                            }
                        }
                    }
                }
                ul {
                    "data-te-dropdown-menu-ref": "",
                    "aria-labelledby": "dropdownMenuButton1",
                    class: "ul-menu hidden overflow-hidden dark:bg-neutral-700 [&[data-te-dropdown-show]]:block",
                    li {
                        class: "li-menu dark:text-neutral-200 dark:hover:bg-neutral-600",
                        Link { to: "/", "Go Home!" }
                    }
                    li {
                        class: "li-menu dark:text-neutral-200 dark:hover:bg-neutral-600",
                        Link { to: "/admin", "Go Admin!" }
                    }
                }
            }
        }
    })
}
