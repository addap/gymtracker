#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::use_set;

use crate::auth::{is_logged_in, is_superuser, store_auth_token, ACTIVE_AUTH_TOKEN};

#[derive(Props)]
pub struct AccessControlProps<'a> {
    // TODO doesn't this also have to be pub?
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

    cx.render(rsx! {
        div {
            button {
                class: "btn btn-outline-danger",
                id: "logout-btn",
                name: "logout-btn",
                onclick: move |_| {
                    auth_setter(None);
                    store_auth_token(None);
                },
                "Logout"
            }
        }
    })
}
