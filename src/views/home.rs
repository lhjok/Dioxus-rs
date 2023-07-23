#![allow(non_snake_case)]
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use dioxus_router::Link;

#[wasm_bindgen(module="\
/static/scripts/home.js")]
extern {
    fn swiperInit();
}

#[wasm_bindgen]
pub fn swiper_init() {
    swiperInit();
}

pub fn Home(cx: Scope) -> Element {
    use_effect(cx, (), |()| async move {
        swiper_init();
    });

    cx.render(rsx!{
        div {
            id: "home",
            header {
                class: "absolute w-full h-16 z-50 bg-white/20 shadow-2xl",
                div {
                    class: "flex flex-row",
                    div {
                        class: "basis-72 ml-6",
                        img { alt: "Logo", src: "./static/images/logo.png" }
                    }
                    div {
                        class: "basis-60 ml-auto py-3",
                        button {
                            class: "mBtn-primary mx-4",
                            Link { to: "/home", "注册用户" }
                        }
                        button {
                            class: "mBtn-primary",
                            Link { to: "/login", "用户登录" }
                        }
                    }
                }
            }
            div {
                class: "swiper w-full",
                div {
                    class: "swiper-wrapper",
                    div {
                        class: "swiper-slide",
                        img { 
                            alt: "Poster01",
                            src: "./static/images/poster01.jpg"
                        }
                    }
                    div {
                        class: "swiper-slide",
                        img {
                            alt: "Poster02",
                            src: "./static/images/poster02.jpg"
                        }
                    }
                }
                div { class: "swiper-pagination" }
                div { class: "swiper-button-prev" }
                div { class: "swiper-button-next" }
            }
        }
    })
}
