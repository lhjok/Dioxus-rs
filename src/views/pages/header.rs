#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::route::Route;

pub fn Header(cx: Scope) -> Element {
    render! {
        div {
            id: "header",
            div {
                class: "logo",
                Link { to: Route::Home {}, h1 { "千鸟任务管理系统" } }
            }
        }
    }
}
