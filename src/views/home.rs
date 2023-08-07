#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use wasm_bindgen::prelude::*;
use crate::route::Route;
use std::rc::Rc;

#[wasm_bindgen(module=
"/static/scripts/swiper.js")]
extern {
    // 导入inti实例对象
    #[wasm_bindgen(js_name= init)]
    static INIT: JsValue;
    // 导出Swiper类型
    type Swiper;
    // 创建Swiper实例(函数)
    #[wasm_bindgen(js_name= newSwiper)]
    fn new_swiper() -> Swiper;
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
    // let swiper= Rc::new(new_swiper());   // 方法一；实例化(函数)
    let swiper= Rc::new(Swiper::new(".swiper", &INIT));   // 方法二；实例化(构造函数)
    let listener= Rc::clone(&swiper);
    let start= Rc::clone(&swiper);

    use_effect(cx, (), |()| async move {
        start.init();
    });

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
                            src: "./static/images/logo.png" }
                    }
                    div {
                        class: "basis-60 ml-auto py-3",
                        button {
                            class: "btn-primary mx-4",
                            Link { to: Route::Home {}, "注册用户" }
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
