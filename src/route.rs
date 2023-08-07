#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::views::{ Admin, Login, Home, Wrong };
use crate::views::pages::content::AdminIndex;
use crate::views::pages::AdminWrong;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/login")]
    Login {},
    #[nest("/admin")]
       #[layout(Admin)]
          #[redirect("/", || Route::AdminIndex {})]
          #[route("/index")]
             AdminIndex {},
          #[route("/404")]
             AdminWrong {},
       #[end_layout]
    #[end_nest]
    #[route("/404")]
    Wrong {}
}
