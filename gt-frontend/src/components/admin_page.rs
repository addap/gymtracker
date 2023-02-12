#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::use_read;

use crate::{api, messages::{MessageProps, UIMessage}, auth::ACTIVE_AUTH_TOKEN, request_ext::RequestExt};
use gt_core::models;

pub fn AdminPage<'a>(cx: Scope<'a, MessageProps<'a>>) -> Element<'a> {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);
    let to_delete = use_state(&cx, || "".to_string());
    let to_expand = use_state(&cx, || "".to_string());

    cx.render(rsx! {
        div {
            label {
                r#for: "to-delete",
                "Name to delete: "
            }
            input {
                id: "to-delete",
                value: "{to_delete}",
                oninput: move |evt| {
                    to_delete.set(evt.value.clone())
                }
            }
            div { class: "w-100" }
            label {
                r#for: "to-expand",
                "Name to merge into: "
            }
            input {
                id: "to-expand",
                value: "{to_expand}",
                oninput: move |evt| {
                    to_expand.set(evt.value.clone())
                }
            }
            div { class: "w-100" }
            button {
                class: "btn btn-outline-danger",
                onclick: move |_| cx.spawn({
                    to_owned![to_delete, to_expand, auth_token];
                    let display_message = cx.props.display_message.clone();

                    async move {
                        let client = reqwest::Client::new();

                        if !to_delete.is_empty() 
                        && !to_expand.is_empty() {
                            let names = models::MergeNames {
                                to_delete: (*to_delete.current()).clone(),
                                to_expand: (*to_expand.current()).clone(),
                            };

                            let res = client.post(api::MERGE_NAMES.as_str())
                                .json(&names).bearer_auth(auth_token.unwrap_or("".into()))
                                .send().await
                                .handle_result::<u64>(UIMessage::error("Merging names failed.".to_string())).await;

                            match res {
                                Ok(rows) => {
                                    display_message.send(UIMessage::info(format!("Updated {} rows.", rows)));
                                }
                                Err(e) => display_message.send(e)
                            }
                        }
                    }
                }),
                "Merge"
            }
        }
    })
}
