#![allow(non_snake_case)]
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use dioxus_router::Link;
use dioxus_web::use_eval;
use web_sys::console;

#[wasm_bindgen(module=
"/static/scripts/login.js")]
extern {
    fn init_te();
}

#[wasm_bindgen]
pub fn init_elements() {
    init_te();
}

pub fn Login(cx: Scope) -> Element {

    let eval = use_eval(&cx);
    eval("console.log('Eval from login.rs')");

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
                    class: "flex mBtn-secondary",
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
                                d: "M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 \
                                    1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z",
                                "fill-rule": "evenodd",
                                "clip-rule": "evenodd"
                            }
                        }
                    }
                }
                ul {
                    "data-te-dropdown-menu-ref": "",
                    "aria-labelledby": "dropdownMenuButton1",
                    class: "ul-menu [&[data-te-dropdown-show]]:block",
                    li {
                        class: "li-menu",
                        Link { to: "/", "Go Home!" }
                    }
                    li {
                        class: "li-menu",
                        Link { to: "/admin", "Go Admin!" }
                    }
                }
            }
        }
    })
}
