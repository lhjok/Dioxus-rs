#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Aside(cx: Scope) -> Element {
    cx.render(rsx!{
        style {
            r#type: "text/css",
            include_str!("./styles/aside.css")
        }
        div {
            id: "aside",
            ul {
                class: "list",
                li { Link { to: "/admin/index", "首页"} }
                li { Link { to: "/admin/config", "管理" } }
            }
        }
    })
}
