#![allow(non_snake_case)]
use const_format::concatcp;
use dioxus::prelude::*;
use dioxus_router::Link;
use fermi::{use_atom_state, Atom};
use gloo_file::{Blob, ObjectUrl};
use log::info;

use crate::components as c;
use crate::{
    api,
    auth::{is_logged_in, is_superuser, ACTIVE_AUTH_TOKEN},
    APP_BASE,
};

pub static USER_PICTURE: Atom<String> = |_| "".to_string();

pub fn Navbar(cx: Scope) -> Element {
    let auth_token = use_atom_state(&cx, ACTIVE_AUTH_TOKEN);
    let user_picture_objecturl = use_state(&cx, || ObjectUrl::from(Blob::new("")));
    let user_picture = use_atom_state(&cx, USER_PICTURE);

    let _fetch_image = use_future(&cx, auth_token, |auth_token_opt| {
        to_owned![user_picture_objecturl, user_picture];
        async move {
            if let Some(auth_token) = auth_token_opt.current().as_ref() {
                let client = reqwest::Client::new();
                let res = client
                    .get(api::USER_PICTURE.as_str())
                    .bearer_auth(auth_token)
                    .send()
                    .await;

                match res {
                    Ok(res) => match res.bytes().await {
                        Ok(bytes) => {
                            let url = ObjectUrl::from(Blob::new(bytes.as_ref()));
                            user_picture.set(url.to_string());
                            user_picture_objecturl.set(url);
                        }
                        Err(e) => {
                            info!("{}", e);
                        }
                    },
                    Err(e) => {
                        info!("{}", e);
                    }
                }
            }
        }
    });

    cx.render(rsx! {
        div {
            class: "sticky-top",
            nav {
                class: "navbar navbar-expand bg-body-secondary",
                div {
                    class: "me-auto navbar-nav",
            if is_logged_in(&cx) {
                rsx!{
                    a {
                        class: "navbar-brand",
                        href: "#",
                        img {
                            src: "{user_picture.current()}",
                            width: 50,
                            height: 50
                        }
                    }
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
                    div {
                        class: "nav-item navbar-text",
                        Link {
                            class: "nav-link",
                            to: concatcp!(APP_BASE, "/user"), "User"
                        }
                    }
                    if is_superuser(&cx) {
                        rsx! {
                            div {
                                class: "nav-item navbar-text",
                                Link {
                                    class: "nav-link",
                                    to: concatcp!(APP_BASE, "/admin"), "Admin"
                                }
                            }
                        }
                    }
                }
            } else {
                rsx! {
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
            }
                }
            }
        }
    })
}
