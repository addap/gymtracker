#![allow(non_snake_case)]
use const_format::concatcp;
use dioxus::prelude::*;
use dioxus_router::Link;
use log::info;
use wasm_cookies::cookies;

use crate::components as c;
use crate::{auth::is_logged_in, BASE_URL};

pub fn MainPage(cx: Scope) -> Element {
    if is_logged_in(&cx) {
        cx.render(rsx! {
            div {
                p { "Main page" }
                c::AddExerciseSetWeighted {}
            }
        })
    } else {
        cx.render(rsx! {
            div {
                p { "Main page" }
                Link { to: concatcp!(BASE_URL, "/register"), "Register"}
                Link { to: concatcp!(BASE_URL, "/login"), "Login" }
            }
        })
    }
}
