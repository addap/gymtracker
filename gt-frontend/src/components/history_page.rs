#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::use_read;
use gt_core::models;
use log::info;

use crate::components as c;
use crate::messages::{MessageProps, UIMessage};
use crate::request_ext::RequestExt;
use crate::{api_url, auth::ACTIVE_AUTH_TOKEN};

pub fn HistoryPage<'a>(cx: Scope<'a, MessageProps<'a>>) -> Element<'a> {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);

    let fetch = use_future(&cx, (), |()| {
        to_owned![auth_token];
        let display_message = cx.props.display_message.clone();

        async move {
            let client = reqwest::Client::new();
            let res = client
                .get(api_url("/exercise/set"))
                .bearer_auth(auth_token.unwrap_or("".into()))
                .send()
                .await
                .handle_result(UIMessage::error(
                    "Requesting exercise history failed.".to_string(),
                ))
                .await;

            match res {
                Ok(history) => history,
                Err(e) => {
                    display_message.send(e);
                    vec![]
                }
            }
        }
    });

    let content = match fetch.value() {
        Some(history) => {
            let hlist = history.iter().map(|exs| {
                rsx! {
                    li { c::ExerciseSet { exs: exs } }
                }
            });
            rsx! {
                button {
                    onclick: move |_| fetch.restart(),
                    "Refresh"
                }
                if !history.is_empty() {
                    rsx!{
                        ul { hlist }
                    }
                }
            }
        }
        None => {
            rsx! {
                p { "Loading" }
            }
        }
    };

    cx.render(rsx! {
        div {
            p { "History page" }
            rsx! { content }
        }
    })
}
