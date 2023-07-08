#![allow(non_snake_case)]
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use dioxus_router::Link;

#[wasm_bindgen(module="/static/scripts/user-module/home.js")]
extern {
    fn swiperInit();
}

#[wasm_bindgen]
pub fn swiper_init() {
    return swiperInit();
}

pub fn Home(cx: Scope) -> Element {

    use_effect(cx, (), |()| async move {
        swiper_init();
    });

    cx.render(rsx!{
        style {
            r#type: "text/css",
            include_str!("./styles/home.css")
        }
        div {
            id: "home",
            header {
                class: "header",
                div {
                    class: "header-row",
                    div {
                        class: "header-logo",
                        img { src: "./images/logo.png" }
                    }
                    div {
                        class: "header-right",
                        button {
                            class: "header-button",
                            Link { to: "/home", "注册用户" }
                        }
                        button {
                            class: "header-button",
                            Link { to: "/login", "用户登录" }
                        }
                    }
                }
            }
            div {
                class: "swiper",
                div {
                    class: "swiper-wrapper",
                    div {
                        class: "swiper-slide",
                        img { src: "./images/poster01.jpg" }
                    }
                    div {
                        class: "swiper-slide",
                        img { src: "./images/poster02.jpg" }
                    }
                }
                div { class: "swiper-pagination" }
                div { class: "swiper-button-prev" }
                div { class: "swiper-button-next" }
            }
        }
    })
}