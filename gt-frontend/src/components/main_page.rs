#![allow(non_snake_case)]
use const_format::concatcp;
use dioxus::prelude::*;
use dioxus_router::Link;

use crate::components as c;
use crate::{auth::is_logged_in, APP_BASE};

pub fn MainPage(cx: Scope) -> Element {
    if is_logged_in(&cx) {
        cx.render(rsx! {
            div {
                p { "Main page" }
                Link { to: concatcp!(APP_BASE, "/history"), "History" }
                br {}
                c::AddExerciseSetWeighted {}
            }
        })
    } else {
        cx.render(rsx! {
            div {
                p { "Main page" }
                Link { to: concatcp!(APP_BASE, "/register"), "Register" }
                br {}
                Link { to: concatcp!(APP_BASE, "/login"), "Login" }
            }
        })
    }
}
