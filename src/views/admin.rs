#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::route::Route;
use super::pages::{
    Aside, Footer, Header, Content
};

pub fn Admin(cx: Scope) -> Element {
    render! {
        Header {}
        Aside {}
        Content {
            Outlet::<Route> {}
        }
        Footer {}
    }
}
