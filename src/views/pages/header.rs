#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx!{
        style {
            r#type: "text/css",
            include_str!("./styles/header.css")
        }
        div {
            id: "header",
            div {
                class: "logo",
                Link { to: "/", h1 { "千鸟任务管理系统" } }
            }
        }
    })
}
