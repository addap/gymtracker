#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::use_read;
use gt_core::models;
use log::info;

use crate::components as c;
use crate::{api_url, auth::ACTIVE_AUTH_TOKEN};

pub fn HistoryPage(cx: Scope) -> Element {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);

    let fetch = use_future(&cx, (), |()| {
        to_owned![auth_token];

        async move {
            let client = reqwest::Client::new();
            let res = client
                .get(api_url("/exercise/set"))
                .bearer_auth(auth_token.unwrap_or("".into()))
                .send()
                .await;

            if let Err(ref e) = res {
                info!("{}", e);
                return vec![];
            }
            let history = res
                .unwrap()
                .json::<Vec<models::ExerciseSetQuery>>()
                .await
                .unwrap();

            history
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
