#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::{use_read, use_set};

use crate::auth::{is_logged_in, set_auth_token, ActiveAuthToken};

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

pub fn Logout(cx: Scope) -> Element {
    let auth_setter = use_set(&cx, ActiveAuthToken);

    cx.render(rsx! {
        div {
            input {
                id: "logout-btn",
                name: "logout-btn",
                r#type: "button",
                value: "Logout",
                onclick: move |_| {
                    set_auth_token(auth_setter, None);
                }
            }
        }
    })
}
