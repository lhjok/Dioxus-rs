#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use wasm_bindgen::prelude::*;
use crate::route::Route;
use js_sys::JSON;
use std::rc::Rc;

#[wasm_bindgen(module="/static/scripts\
/swiper-bundle.esm.browser.min.js")]
extern {
    // 导出Swiper类型
    type Swiper;
    // 创建Swiper实例(构造函数)
    #[wasm_bindgen(constructor)]
    fn new(id: &str, init: &JsValue) -> Swiper;
    // Swipe清理事件监听器方法
    #[wasm_bindgen(method, js_name= detachEvents)]
    fn detach_events(this: &Swiper);
    // Swipe初始化方法
    #[wasm_bindgen(method)]
    fn init(this: &Swiper);
}

pub fn Home(cx: Scope) -> Element {
    // 解析Json格式到JsValue对象
    let init = JSON::parse(r#"{
        "init": false,
        "direction": "horizontal",
        "pagination": {
            "el": ".swiper-pagination"
        },
        "navigation": {
            "nextEl": ".swiper-button-next",
            "prevEl": ".swiper-button-prev"
        }
    }"#).unwrap();
    // 实例化(构造函数)
    let swiper = Rc::new(Swiper::new(".swiper", &init));
    let listener = Rc::clone(&swiper);
    let start = Rc::clone(&swiper);
    // 挂载组件时调用一次。
    use_effect(cx, (), |()| async move {
        start.init();
    });
    // 卸载组件时调用一次。
    use_on_unmount(cx, move || {
        listener.detach_events();
    });

    render! {
        div {
            id: "home",
            header {
                class: "absolute w-full h-16 z-50 bg-white/20 shadow-2xl",
                div {
                    class: "flex flex-row",
                    div {
                        class: "basis-72 ml-6",
                        img {
                            alt: "Logo",
                            src: "./static/images/logo.png"
                        }
                    }
                    div {
                        class: "basis-60 ml-auto py-3",
                        button {
                            class: "btn-primary mx-4",
                            Link { to: Route::Wrong {}, "注册用户" }
                        }
                        button {
                            class: "btn-primary",
                            Link { to: Route::Login {}, "用户登录" }
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
    }
}
