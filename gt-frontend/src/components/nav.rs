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
                        class: "navbar navbar-expand bg-body-tertiary",
                        div {
                            class: "container-fluid",
                                div {
                                    class: "navbar-nav",
                                    div {
                                        class: "nav-item",
                                        div {
                                            class: "nav-link",
                                            c::Logout {}
                                        }
                                    }
                                    div {
                                        class: "nav-item navbar-text",
                                        Link {
                                            class: "nav-link",
                                            to: concatcp!(APP_BASE, "/"), "Home"
                                        }
                                    }
                                    div {
                                        class: "nav-item navbar-text",
                                        Link {
                                            class: "nav-link",
                                            to: concatcp!(APP_BASE, "/history"), "History"
                                        }
                                    }
                                    div {
                                        class: "nav-item navbar-text",
                                        Link {
                                            class: "nav-link",
                                            to: concatcp!(APP_BASE, "/pr"), "PR"
                                        }
                                    }
                                }
                        }
                    }
                }
            } else {
                rsx!{
                    nav {
                        class: "navbar navbar-expand-lg bg-body-tertiary",
                        // div {
                        //     class: "navbar-collapse collapse w-100",
                            div {
                                class: "navbar-nav me-auto",
                                div {
                                    class: "nav-item",
                                    Link {
                                        class: "nav-link",
                                        to: concatcp!(APP_BASE, "/register"), "Register"
                                    }
                                }
                                div {
                                    class: "nav-item",
                                    Link {
                                        class: "nav-link",
                                        to: concatcp!(APP_BASE, "/login"), "Login"
                                    }
                                }
                            }
                        // }
                    }
                }
            }
        }
    })
}
