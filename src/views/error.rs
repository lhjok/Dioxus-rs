#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Error(cx: Scope) -> Element {
    cx.render(rsx!{
        style { include_str!("./styles/error.css") }
        div {
            id: "error",
            h3 { "404 Error" }
            p { "您访问的页面已经不存在了..." }
            Link { to: "/", "返回首页" }
        }
    })
}
