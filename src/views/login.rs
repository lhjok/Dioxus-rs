#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use wasm_bindgen::prelude::*;
use crate::route::Route;
use web_sys::Element as WebSysElement;
use gloo::events::EventListener;
use gloo_utils::document;

#[wasm_bindgen(module="/node_modules\
/tw-elements/dist/js/tw-elements.es.min.js")]
extern {
    type Input;
    #[wasm_bindgen(constructor)]
    fn new(el: &WebSysElement) -> Input;
    // #[wasm_bindgen(method)]
    // fn update(this: &Input);

    type Dropdown;
    #[wasm_bindgen(constructor)]
    fn new(el: &WebSysElement) -> Dropdown;
    #[wasm_bindgen(method)]
    fn toggle(this: &Dropdown);
}

fn init_input(id: &str) -> Result<Vec<(WebSysElement, Input)>, JsValue> {
    let trigger = document().query_selector_all(id)?;
    let mut inputs: Vec<(WebSysElement, Input)> = Vec::new();
    for i in 0..trigger.length() {
        let element = trigger.item(i).unwrap().unchecked_into();
        let input = Input::new(&element);
        inputs.push((element, input));
    }
    Ok(inputs)
}

fn init_dropdown(id: &str) -> Result<Vec<(WebSysElement, Dropdown)>, JsValue> {
    let trigger = document().query_selector_all(id)?;
    let mut dropdowns: Vec<(WebSysElement, Dropdown)> = Vec::new();
    for i in 0..trigger.length() {
        let element = trigger.item(i).unwrap().unchecked_into();
        let dropdown = Dropdown::new(&element);
        dropdowns.push((element, dropdown));
    }
    Ok(dropdowns)
}

pub fn Login(cx: Scope) -> Element {
    use_effect(cx, (), |()| async move {
        let _ = init_input("[data-te-input-wrapper-init]").unwrap();
        let dropdowns = init_dropdown("[data-te-dropdown-toggle-ref]").unwrap();
        for (element, dropdown) in dropdowns.into_iter() {
            let event = EventListener::new(&element, "click", move |_event| {
                // event.prevent_default();
			    dropdown.toggle();
		    });
            event.forget();
        }
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
                            div {
                                class: "relative",
                                "data-te-dropdown-ref": "",
                                button {
                                    r#type: "button",
                                    id: "dropdownMenuButton1",
                                    "data-te-dropdown-toggle-ref": "",
                                    "aria-expanded": "false",
                                    class: "flex w-full items-center justify-center btn-secondary",
                                    style: "background-color: #55acee",
                                    "导航菜单",
                                    span {
                                        class: "ml-2 w-2",
                                        svg {
                                            class: "h-5 w-5",
                                            fill: "currentColor",
                                            "viewBox": "0 0 20 20",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            path {
                                                d: "M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 \
                                                    1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z",
                                                "fill-rule": "evenodd",
                                                "clip-rule": "evenodd"
                                            }
                                        }
                                    }
                                }
                                ul {
                                    "data-te-dropdown-menu-ref": "",
                                    "aria-labelledby": "dropdownMenuButton1",
                                    class: "ul-menu w-full [&[data-te-dropdown-show]]:block",
                                    li {
                                        class: "li-menu text-center",
                                        Link { to: Route::Home {}, "返回首页" }
                                    }
                                    li {
                                        class: "li-menu text-center",
                                        Link { to: Route::AdminIndex {}, "后台管理" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
