#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::route::Route;

pub fn Login(cx: Scope) -> Element {
    render! {
        div {
            id: "login",
            header { class: "absolute w-full h-16 z-50 bg-white/20 shadow-2xl",
                div { class: "flex flex-row",
                    div { class: "basis-72 ml-6", img { alt: "Logo", src: "./static/images/logo.png" } }
                    div { class: "basis-60 ml-auto py-3",
                        button { class: "btn-primary mx-4", Link { to: Route::Home {}, "返回首页" } }
                        button { class: "btn-primary", Link { to: Route::AdminIndex {}, "用户登录" } }
                    }
                }
            }
        }
    }
}
