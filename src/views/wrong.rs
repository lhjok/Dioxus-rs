#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::route::Route;

pub fn Wrong(cx: Scope) -> Element {
    render! {
        div {
            id: "wrong",
            h3 { "404 Error" }
            p { "您访问的页面已经不存在了..." }
            Link { to: Route::Home {}, "返回首页" }
        }
    }
}
