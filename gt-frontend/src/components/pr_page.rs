#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::use_read;
use itertools::join;

use crate::messages::{MessageProps, UIMessage};
use crate::request_ext::RequestExt;
use crate::{api, auth::ACTIVE_AUTH_TOKEN};
use gt_core::models;

pub fn PRPage<'a>(cx: Scope<'a, MessageProps<'a>>) -> Element<'a> {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);
    let search_term = use_state(&cx, || "".to_string());

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
            let prlist_weighted = prs.weighted.iter().filter(|pr| {
                let name = pr.name.to_lowercase();
                let search = (*search_term.current()).clone();
                name.contains(&search)
            }).map(|pr| {
                rsx! {
                    li { format!("{}: [ {} ]", pr.name.clone(), join(pr.pr.iter()
                            .map(|(weight, reps)| format!("{} × {:.1}kg", reps, weight)), " | ")) }
                }
            });
            let prlist_bodyweight = prs
                .bodyweight
                .iter()
                .filter(|pr| {
                    let name = pr.name.to_lowercase();
                    let search = (*search_term.current()).clone();
                    name.contains(&search)
                })
                .map(|pr| {
                    rsx! {
                        li { format!("{}: [ {} ]", pr.name.clone(), join(pr.pr.iter()
                                .map(|reps| format!("{} × 身", reps)), " | ")) }
                    }
                });
            rsx! {
                div {
                    class: "my-3 p-2",
                    form {
                        class: "row g-1 g-sm-2",
                        div {
                            class: "form-group col-12 col-sm-auto",
                            input {
                                id: "pr-search-box",
                                class: "form-control",
                                value: "{ search_term }",
                                placeholder: "Search",
                                oninput: move |evt| { search_term.set(evt.value.to_lowercase()) }
                            }
                        }
                        div {
                            class: "form-group col-12 col-sm-auto",
                            button {
                                class: "btn btn-outline-secondary",
                                r#type: "button",
                                onclick: move |_| fetch.restart(),
                                "Refresh"
                            }
                        }
                    }
                    div {
                        class: "bg-body-tertiary",
                        p { "By Weight" }
                        ul { prlist_weighted }
                    }
                    div {
                        class: "bg-body-tertiary",
                        p { "By Bodyweight" }
                        ul { prlist_bodyweight }
                    }
                }
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
