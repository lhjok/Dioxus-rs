#![allow(non_snake_case)]
pub mod index;
pub use self::index::AdminIndex;
use dioxus::prelude::*;

#[derive(Props)]
pub struct Props<'a> {
    pub children: Element<'a>
}

pub fn Content<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    render! {
        div {
            id: "content",
            &cx.props.children
        }
    }
}
