#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx!{
        style { include_str!("./styles/home.css") }
        div {
            ul {
                li { Link { to: "/login", "Go Login!"} }
            }
        }
    })
}
