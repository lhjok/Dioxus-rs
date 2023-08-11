#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use wasm_bindgen::prelude::*;
use crate::route::Route;
use web_sys::Element as WebSysElement;
use gloo::events::EventListener;
use gloo_utils::document;

#[wasm_bindgen(module=
"/static/scripts/elements.js")]
extern {
    type Input;
    #[wasm_bindgen(constructor)]
    fn new(el: &WebSysElement) -> Input;
    #[wasm_bindgen(method)]
    fn update(this: &Input);
    
    // #[wasm_bindgen(js_name= initInput)]
    // fn init_input(id: &str);
    // #[wasm_bindgen(js_name= initDropdown)]
    // fn init_dropdown(id: &str);
}

fn init_input(id: &str) {
    let trigger = document().query_selector_all(id).unwrap();
    for i in 0..trigger.length() {
        let element = trigger.item(i).unwrap().unchecked_into();
        let input = Input::new(&element);
        let _event = EventListener::new(&element, "click", move |event| {
            event.prevent_default();
			input.update();
		});
    }
}

pub fn Login(cx: Scope) -> Element {
    use_effect(cx, (), |()| async move {
        init_input("[data-te-input-wrapper-init]");
        // init_dropdown("[data-te-dropdown-toggle-ref]");
    });
    render! {
        section {
            class: "h-screen",
            div {
                class: "container h-full px-6 py-24 mx-auto max-w-7xl",
                div {
                    class: "g-6 flex h-full flex-wrap items-center justify-center lg:justify-between",
                    div {
                        class: "mb-12 md:mb-0 md:w-8/12 lg:w-6/12",
                        img {
                            class: "w-full",
                            src: "./static/images/draw2.webp",
                            alt: "Phone image"
                        }
                    }
                    div {
                        class: "md:w-8/12 lg:ml-6 lg:w-5/12",
                        form {
                            div {
                                class: "relative mb-6",
                                "data-te-input-wrapper-init": "",
                                input {
                                    r#type: "text",
                                    class: "peer m-input",
                                    id: "exampleFormControlInput3",
                                    placeholder: "Email address"
                                }
                                label {
                                    r#for: "exampleFormControlInput3",
                                    class: "m-label",
                                    "帐号"
                                }
                            }
                            div {
                                class: "relative mb-6",
                                "data-te-input-wrapper-init": "",
                                input {
                                    r#type: "password",
                                    class: "peer m-input",
                                    id: "exampleFormControlInput33",
                                    placeholder: "Password"
                                }
                                label {
                                    r#for: "exampleFormControlInput33",
                                    class: "m-label",
                                    "密码"
                                }
                            }
                            div {
                                class: "mb-6 flex items-center justify-between",
                                div {
                                    class: "mb-[0.125rem] block min-h-[1.5rem] pl-[1.5rem]",
                                    input {
                                        class: "c-input",
                                        r#type: "checkbox",
                                        id: "exampleCheck3",
                                        value: "",
                                        checked: "",
                                    }
                                    label {
                                        class: "inline-block pl-[0.15rem] hover:cursor-pointer",
                                        r#for: "exampleCheck3",
                                        "记住帐号"
                                    }
                                }
                                a {
                                    href: "#!",
                                    class: "m-link",
                                    "您忘记密码了吗?"
                                }
                            }
                            button {
                                r#type: "submit",
                                class: "inline-block w-full items-center justify-center btn-secondary",
                                "登录"
                            }
                            div {
                                class: "my-4 flex items-center before:mt-0.5 before:flex-1 before:border-t after:flex-1 \
                                        before:border-neutral-300 after:mt-0.5 after:border-t after:border-neutral-300",
                                p {
                                    class: "mx-4 mb-0 text-center font-semibold dark:text-neutral-200",
                                    "OR"
                                }
                            }
                            button {
                                class: "mb-5 flex w-full items-center justify-center btn-secondary",
                                style: "background-color: #3b5998",
                                Link { to: Route::Home {}, "注册用户" }
                            }
                            button {
                                class: "flex w-full items-center justify-center btn-secondary",
                                style: "background-color: #55acee",
                                Link { to: Route::Home {}, "返回首页" }
                            }
                        }
                    }
                }
            }
        }
    }
}
