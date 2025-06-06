#![allow(non_snake_case)]

use bdk::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

pub fn App() -> Element {
    let mut name = use_signal(|| String::new());
    let mut greet_msg = use_signal(|| String::new());

    let greet = move |_: FormEvent| async move {
        if name.read().is_empty() {
            return;
        }

        let name = name.read();
        let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &*name }).unwrap();
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        let new_msg = invoke("greet", args).await.as_string().unwrap();
        greet_msg.set(new_msg);
    };

    rsx! {
        link { rel: "stylesheet", href: "styles.css" }
        main { class: "container",
            h1 { "Welcome to Tauri + Dioxus" }

            div { class: "row",
                a { href: "https://tauri.app", target: "_blank",
                    img {
                        src: asset!("/public/tauri.svg"),
                        class: "logo tauri",
                        alt: "Tauri logo",
                    }
                }
                a { href: "https://dioxuslabs.com/", target: "_blank",
                    img {
                        src: asset!("/public/dioxus.png"),
                        class: "logo dioxus",
                        alt: "Dioxus logo",
                    }
                }
            }
            p { "Click on the Tauri and Dioxus logos to learn more." }

            form { class: "row", onsubmit: greet,
                input {
                    id: "greet-input",
                    placeholder: "Enter a name...",
                    value: "{name}",
                    oninput: move |event| name.set(event.value()),
                }
                button { r#type: "submit", "Greet" }
            }
            p { "{greet_msg}" }
        }
    }
}