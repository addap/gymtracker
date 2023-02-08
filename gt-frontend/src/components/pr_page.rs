#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::use_read;
use gt_core::models;
use log::info;

use crate::messages::{MessageProps, UIMessage};
use crate::request_ext::RequestExt;
use crate::{api, auth::ACTIVE_AUTH_TOKEN};

pub fn PRPage<'a>(cx: Scope<'a, MessageProps<'a>>) -> Element<'a> {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);

    let fetch = use_future(&cx, (), |()| {
        to_owned![auth_token];
        let display_message = cx.props.display_message.clone();

        async move {
            let client = reqwest::Client::new();
            let res = client
                .get(api::EXERCISE_PR.as_str())
                .bearer_auth(auth_token.unwrap_or("".into()))
                .send()
                .await
                .handle_result::<models::PRQuery>(UIMessage::error(
                    "Fetching PRs failed.".to_string(),
                ))
                .await;

            match res {
                Ok(prs) => Some(prs),
                Err(e) => {
                    display_message.send(e);
                    None
                }
            }
        }
    });

    let content = match fetch.value() {
        Some(Some(prs)) => {
            let prlist = prs.weighted.iter().map(|pr| {
                rsx! {
                    li { format!("{}: {:?}", pr.name.clone(), pr.pr_weight.clone()) }
                }
            });
            rsx! {
                button {
                    onclick: move |_| fetch.restart(),
                    "Refresh"
                }
                // if !prlist.is_empty() {
                    rsx!{
                        ul { prlist }
                    }
                // }
            }
        }
        _ => {
            rsx! {
                p { "Loading" }
            }
        }
    };

    cx.render(rsx! {
        div {
            p { "PR page" }
            rsx! { content }
        }
    })
}
