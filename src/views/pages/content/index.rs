#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Index(cx: Scope) -> Element {
    cx.render(rsx!{
        div {
            id: "admindex",
            h3 { "Index Module" }
            p { "页面正在建设当中..." }
        }
    })
}
