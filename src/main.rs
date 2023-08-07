#![allow(non_snake_case)]
mod views;
mod route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use route::Route;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}
