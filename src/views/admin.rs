#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::{Redirect, use_route};
use crate::views::Error;
use super::pages::content::Index;
use super::pages::{
    Aside, Footer, Header, Content
};

pub fn Admin(cx: Scope) -> Element {
    let route = use_route(&cx);
    cx.render(rsx!{
        Header {}
        Aside {}
        Content {
            match route.segment("route") {
                Some(val) => match val {
                    "index" => rsx!{ Index {} },
                    "404" => rsx!{ Error {} },
                    _ => rsx!{
                        Redirect { to: "/admin/404" }
                    }
                },
                None => rsx!{
                    Redirect { to: "/admin/404" }
                }
            }
        }
        Footer {}
    })
}
