#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::route::Route;

pub fn Aside(cx: Scope) -> Element {
    render! {
        div {
            id: "aside",
            ul {
                class: "list",
                li { Link { to: Route::AdminIndex {}, "首页"} }
                li { Link { to: Route::AdminWrong {}, "管理" } }
            }
        }
    }
}
