#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::prelude::*;



pub fn NewChat(cx:Scope) -> Element {
    cx.render(rsx! {
        form {
            class: "flex flex-col gap-4",
            onsubmit: move |_| (),
            prevent_default: "onsubmit",
            button {
                class: "btn",
                r#type: "submit",
                disabled: true,
                "Post"
            }
        }
    })
}