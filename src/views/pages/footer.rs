#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx!{
        style { include_str!("./styles/footer.css") }
        div {
            id: "footer",
            p { "千鸟科技 - 瑞金市千鸟网络科技有限公司 CopyRight 2017-2020" }
            span { " - 赣ICP备17011754号-2 - " } " 软件著作权 - "
            span { "软著登字第8888888号" }
        }
    })
}
