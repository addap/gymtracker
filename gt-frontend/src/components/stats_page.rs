#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn StatsPage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            p { "Stats page" }
        }
    })
}
