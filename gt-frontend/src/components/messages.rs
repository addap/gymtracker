#![allow(non_snake_case)]
use std::collections::VecDeque;

use dioxus::prelude::*;

use crate::messages::{UIMessage, UIMessageType};

#[inline_props]
pub fn Messages<'a>(cx: Scope, ui_messages: &'a UseRef<VecDeque<UIMessage>>) -> Element<'a> {
    // let message_items = ;

    cx.render(rsx! {
        ui_messages.read().iter().map(|ui_message| {
            match ui_message.r#type {
                UIMessageType::Info =>
                    rsx! {
                        div {
                            class: "alert alert-success alert-dismissible",
                            div { ui_message.message.clone() }
                        }
                    },
                UIMessageType::Error =>
                    rsx! {
                        div {
                            class: "alert alert-danger alert-dismissible",
                            div { format!("Error: {}", ui_message.message) }
                        }
                    },
            }
        })
    })
}
