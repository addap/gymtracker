#![allow(non_snake_case)]
use const_format::concatcp;
use dioxus::prelude::*;
use dioxus_router::Link;

use crate::components as c;
use crate::{auth::is_logged_in, APP_BASE};

pub fn Navbar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "sticky-top",
            if is_logged_in(&cx) {
                rsx!{
                    nav {
                        class: "navbar navbar-expand-lg bg-body-tertiary",
                        div {
                            class: "container-fluid",
                            div {
                                class: "nav-link",
                                c::Logout {}
                            }
                            div {
                                class: "nav-link",
                                Link { to: concatcp!(APP_BASE, "/"), "Home" }
                            }
                            div {
                                class: "nav-link",
                                Link { to: concatcp!(APP_BASE, "/history"), "History" }
                            }
                            div {
                                class: "nav-link",
                                Link { to: concatcp!(APP_BASE, "/pr"), "PR" }
                            }
                        }
                    }
                }
            }
        }
    })
}
