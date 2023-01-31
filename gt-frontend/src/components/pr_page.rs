#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::use_read;
use gt_core::models;
use log::info;

use crate::components as c;
use crate::{api_url, auth::ACTIVE_AUTH_TOKEN};

pub fn PRPage(cx: Scope) -> Element {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);

    let fetch = use_future(&cx, (), |()| {
        to_owned![auth_token];

        async move {
            let client = reqwest::Client::new();
            let res = client
                .get(api_url("/exercise/pr"))
                .bearer_auth(auth_token.unwrap_or("".into()))
                .send()
                .await;

            if let Err(ref e) = res {
                info!("{}", e);
                return None;
            }
            let prs = res.unwrap().json::<models::PRQuery>().await.unwrap();

            Some(prs)
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
