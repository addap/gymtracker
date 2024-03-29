#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::use_router;
use fermi::use_set;
use gt_core::APP_BASE;

use crate::{
    auth::{is_logged_in, is_superuser, store_auth_token, ACTIVE_AUTH_TOKEN},
    components::nav,
};

#[derive(Props)]
pub struct AccessControlProps<'a> {
    children: Element<'a>,
}

pub fn LoggedIn<'a>(cx: Scope<'a, AccessControlProps<'a>>) -> Element<'a> {
    let is_logged_in = is_logged_in(&cx);

    cx.render(rsx! {
        div {
            if is_logged_in {
                rsx!{ &cx.props.children }
            } else {
                rsx!{ p { "You are not logged in. Go back." } }
            }
        }
    })
}

pub fn LoggedOut<'a>(cx: Scope<'a, AccessControlProps<'a>>) -> Element<'a> {
    let is_logged_in = is_logged_in(&cx);

    cx.render(rsx! {
        div {
            if is_logged_in {
                rsx!{ p { "You are already logged in. Go back." } }
            } else {
                rsx!{ &cx.props.children }
            }
        }
    })
}

pub fn Superuser<'a>(cx: Scope<'a, AccessControlProps<'a>>) -> Element<'a> {
    let is_superuser = is_superuser(&cx);

    cx.render(rsx! {
        div {
            if is_superuser {
                rsx!{ &cx.props.children }
            } else {
                rsx!{ p { "You must be an admin to view this site. Go back." } }
            }
        }
    })
}

pub fn Logout(cx: Scope) -> Element {
    let auth_setter = use_set(&cx, ACTIVE_AUTH_TOKEN);
    let router = use_router(&cx);

    cx.render(rsx! {
        div {
            button {
                class: "btn btn-outline-danger",
                id: "logout-btn",
                name: "logout-btn",
                onclick: move |_| {
                    // Remove the auth token from both local storage & the Atom.
                    auth_setter(None);
                    store_auth_token(None);
                    nav::reset_user_picture(&cx);

                    router.navigate_to(APP_BASE);
                },
                "Logout"
            }
        }
    })
}
