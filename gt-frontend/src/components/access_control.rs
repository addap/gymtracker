#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props)]
pub struct AccessControlProps<'a> {
    // TODO doesn't this also have to be pub?
    children: Element<'a>,
}

pub fn LoggedIn<'a>(cx: Scope<'a, AccessControlProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            p { "Please login" }
            &cx.props.children
        }
    })
}

pub fn LoggedOut<'a>(cx: Scope<'a, AccessControlProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            p { "You are already logged in" }
            &cx.props.children
        }
    })
}
