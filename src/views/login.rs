#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Login(cx: Scope) -> Element {
    cx.render(rsx!{
        style {
            r#type: "text/css",
            include_str!("./styles/login.css")
        }
        div {
            id: "login",
            div {
                class: "dropdown",
                button {
                    tabindex: 0,
                    class: "m-1 mBtn-primary", "登录"
                }
                ul {
                    tabindex: 0,
                    class: "p-2 shadow menu dropdown-content \
                    z-[1] bg-base-100 rounded-box w-52",
                    li { Link { to: "/", "Go Home!" } }
                    li { Link { to: "/admin", "Go Admin!" } }
                }
            }
            label {
                r#for: "my_modal_6",
                class: "mBtn-primary",
                "Open Modal"
            }
            input {
                r#type: "checkbox",
                id: "my_modal_6",
                class: "modal-toggle"
            }
            div {
                class: "modal",
                div {
                    class: "modal-box",
                    h3 {
                        class: "font-bold text-lg",
                        "Modal Title"
                    }
                    p {
                        class: "py-4",
                        "Press ESC key or click the button below to close"
                    }
                    div {
                        class: "modal-action",
                        label {
                            r#for: "my_modal_6",
                            class: "mBtn-primary",
                            "Close"
                        }
                    }
                }
            }
        }
    })
}