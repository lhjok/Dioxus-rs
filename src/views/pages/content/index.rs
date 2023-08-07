#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn AdminIndex(cx: Scope) -> Element {
    render! {
        div {
            id: "admindex",
            h3 { "Index Module" }
            p { "页面正在建设当中..." }
        }
    }
}
