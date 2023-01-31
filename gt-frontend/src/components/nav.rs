#![allow(non_snake_case)]
use const_format::concatcp;
use dioxus::prelude::*;
use dioxus_router::Link;

use crate::components as c;
use crate::{auth::is_logged_in, APP_BASE};

pub fn Navbar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            if is_logged_in(&cx) {
                rsx!{
                    c::Logout {}
                    Link { to: concatcp!(APP_BASE, "/"), "Home" }
                    Link { to: concatcp!(APP_BASE, "/history"), "History" }
                    Link { to: concatcp!(APP_BASE, "/pr"), "PR" }
                }
            }
        }
    })
}
