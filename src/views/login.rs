#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Login(cx: Scope) -> Element {
    cx.render(rsx!{
        style { include_str!("./styles/login.css") }
        div {
            ul {
                li { Link { to: "/", "Go Home!"} }
                li { Link { to: "/admin", "Go Admin!" } }
            }
        }
    })
}
