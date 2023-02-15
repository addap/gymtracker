#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::use_read;
use futures_util::StreamExt;
use log::info;

use crate::components as c;
use crate::messages::{MessageProps, UIMessage};
use crate::request_ext::RequestExt;
use crate::{api, auth::ACTIVE_AUTH_TOKEN, PAGE_SIZE};

pub fn HistoryPage<'a>(cx: Scope<'a, MessageProps<'a>>) -> Element<'a> {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);
    let history = use_state(&cx, || Vec::new());

    let fetch = use_coroutine(&cx, |mut rx: UnboundedReceiver<Option<u64>>| {
        to_owned![auth_token, history];
        let display_message = cx.props.display_message.clone();

        async move {
            while let Some(page_size_opt) = rx.next().await {
                let url = match page_size_opt {
                    Some(page_size) => format!("{}/{}", api::EXERCISE_SET.as_str(), page_size),
                    None => api::EXERCISE_SET.to_string(),
                };
                let client = reqwest::Client::new();
                let res = client
                    .get(url)
                    .bearer_auth(auth_token.clone().unwrap_or("".into()))
                    .send()
                    .await
                    .handle_result(UIMessage::error(
                        "Requesting exercise history failed.".to_string(),
                    ))
                    .await;

                match res {
                    Ok(h) => history.set(h),
                    Err(e) => {
                        display_message.send(e);
                    }
                }
            }
        }
    });
    use_future(&cx, (), |()| {
        to_owned![fetch];
        async move { fetch.send(Some(*PAGE_SIZE)) }
    });

    let content = {
        let hlist = history.get().iter().map(|exs| {
            rsx! {
                li { c::ExerciseSet { exs: exs } }
            }
        });
        rsx! {
            button {
                onclick: move |_| fetch.send(None),
                "Show All"
            }
            if !history.current().is_empty() {
                rsx!{
                    ul { hlist }
                }
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
