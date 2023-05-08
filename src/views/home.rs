#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx!{
        style {
            r#type: "text/css",
            include_str!("./styles/home.css")
        }
        div {
            id: "home",
            ul {
                li { Link { to: "/login", "Go Login!" } }
                li { Link { to: "/home", "Go Home!" } }
            }
            div {
                class: "swiper",
                div {
                    class: "swiper-wrapper",
                    div {
                        class: "swiper-slide",
                        img { src: "./images/2.jpg" }
                    }
                    div {
                        class: "swiper-slide",
                        img { src: "./images/3.jpg" }
                    }
                }
                div { class: "swiper-pagination" }
                div { class: "swiper-button-prev" }
                div { class: "swiper-button-next" }
            }
        }
        script {
            src: "./scripts/home.js",
            r#type: "module"
        }
    })
}
