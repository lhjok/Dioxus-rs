#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use wasm_bindgen::prelude::*;
use crate::route::Route;

#[wasm_bindgen(module=
"/static/scripts/elements.js")]
extern {
    #[wasm_bindgen(js_name= initDropdown)]
    fn init_dropdown(id: &str);
}

pub fn Login(cx: Scope) -> Element {
    use_effect(cx, (), |()| async move {
        init_dropdown("[data-te-dropdown-toggle-ref]");
    });

    // use_on_unmount(cx, move || {

    // });

    render! {
        div {
            id: "login",
            div {
                class: "m-8 relative",
                "data-te-dropdown-ref": "",
                button {
                    r#type: "button",
                    id: "dropdownMenuButton1",
                    "data-te-dropdown-toggle-ref": "",
                    "aria-expanded": "false",
                    class: "flex btn-secondary",
                    "Dropdown button",
                    span {
                        class: "ml-2 w-2",
                        svg {
                            class: "h-5 w-5",
                            fill: "currentColor",
                            "viewBox": "0 0 20 20",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                d: "M5.23 7.21a.75.75 0 011.06.02L10 \
                                    11.168l3.71-3.938a.75.75 0 111.08 \
                                    1.04l-4.25 4.5a.75.75 0 01-1.08 \
                                    0l-4.25-4.5a.75.75 0 01.02-1.06z",
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
                        Link { to: Route::Home {}, "Go Home!" }
                    }
                    li {
                        class: "li-menu",
                        Link { to: Route::AdminIndex {}, "Go Admin!" }
                    }
                }
            }
            div {
                class: "m-8 relative",
                "data-te-dropdown-ref": "",
                button {
                    r#type: "button",
                    id: "dropdownMenuButton1",
                    "data-te-dropdown-toggle-ref": "",
                    "aria-expanded": "false",
                    class: "flex btn-secondary",
                    "Dropdown button",
                    span {
                        class: "ml-2 w-2",
                        svg {
                            class: "h-5 w-5",
                            fill: "currentColor",
                            "viewBox": "0 0 20 20",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                d: "M5.23 7.21a.75.75 0 011.06.02L10 \
                                    11.168l3.71-3.938a.75.75 0 111.08 \
                                    1.04l-4.25 4.5a.75.75 0 01-1.08 \
                                    0l-4.25-4.5a.75.75 0 01.02-1.06z",
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
                        Link { to: Route::Home {}, "Go Home!" }
                    }
                    li {
                        class: "li-menu",
                        Link { to: Route::AdminIndex {}, "Go Admin!" }
                    }
                }
            }
        }
    }
}
