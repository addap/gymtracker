#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn RegisterPage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            p { "Register page" }
        }
    })
}
