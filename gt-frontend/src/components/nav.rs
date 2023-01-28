#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::auth::is_logged_in;
use crate::components as c;

pub fn Navbar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            if is_logged_in(&cx) {
                rsx!{ c::Logout {} }
            }
        }
    })
}
