#![allow(non_snake_case)]
use const_format::concatcp;
use derive_more::Deref;
use dioxus::prelude::*;
use dioxus_router::Link;
use fermi::{use_atom_state, use_set, Atom};
use log::info;
use reqwest::StatusCode;

use crate::{
    api,
    auth::{is_logged_in, is_superuser, ACTIVE_AUTH_TOKEN},
    APP_BASE, LOGO,
};
use crate::{components as c, to_dataurl};

#[derive(Deref)]
pub struct WrapperUserPicture<T>(pub T);
pub static USER_PICTURE: Atom<WrapperUserPicture<String>> = |_| WrapperUserPicture((*LOGO).clone());

pub fn reset_user_picture<'a, T: 'a>(cx: &'a Scope<'a, T>) {
    let user_picture = use_set(cx, USER_PICTURE);
    user_picture(WrapperUserPicture((*LOGO).clone()));
}

pub fn Navbar(cx: Scope) -> Element {
    let auth_token = use_atom_state(&cx, ACTIVE_AUTH_TOKEN);
    let user_picture = use_atom_state(&cx, USER_PICTURE);

    // Asynchronously fetch user picture and set the Atom.
    // Somehow the whole site broke when the USER_PICTURE atom was in the main_page module, so for now we keep it in the navbar module.
    let _fetch_image = use_future(&cx, auth_token, |auth_token_opt| {
        to_owned![user_picture, user_picture];
        async move {
            if let Some(auth_token) = auth_token_opt.current().as_ref() {
                let client = reqwest::Client::new();
                let res = client
                    .get(api::USER_PICTURE.as_str())
                    .bearer_auth(auth_token)
                    .send()
                    .await;

                match res {
                    Ok(res) => {
                        if res.status() == StatusCode::OK {
                            match res.bytes().await {
                                Ok(bytes) => {
                                    info!("Fetched user image.");
                                    let data_url = to_dataurl(bytes.as_ref());
                                    user_picture.set(WrapperUserPicture(data_url));
                                }
                                Err(e) => {
                                    info!("{}", e);
                                }
                            }
                        }
                    }
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
                    div {
                        class: "navbar-brand",
                        img {
                            src: "{user_picture.current().0}",
                            width: 50,
                            height: 50
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
                            to: concatcp!(APP_BASE, "/graph"), "Graph"
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
                    div {
                        class: "nav-item",
                        div {
                            class: "nav-link",
                            c::Logout {}
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
