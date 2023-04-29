#![allow(non_snake_case)]
mod views;
use views::{ Admin, Login, Home, Error };
use dioxus::prelude::*;
use dioxus_router::{ Route, Router, Redirect };

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx!{
        Router {
            Route { to: "/", Home {} }
            Route { to: "/login", Login {} }
            Route { to: "/admin", Redirect { to: "/admin/index" } }
            Route { to: "/admin/:route", Admin {} }
            Route { to: "", Error {} }
        }
    })
}
