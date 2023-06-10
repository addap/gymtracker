#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::use_read;
use gt_core::models;

use crate::{
    api,
    auth::{self, ACTIVE_AUTH_TOKEN},
    messages::{MessageProps, UIMessage},
    request_ext::RequestExt,
};

fn MergeNames<'a>(cx: Scope<'a, MessageProps<'a>>) -> Element<'a> {
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

fn ResetPassword<'a>(cx: Scope<'a, MessageProps<'a>>) -> Element<'a> {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);
    let username = use_state(&cx, || "".to_string());
    let password = use_state(&cx, || "".to_string());

    cx.render(rsx! {
        div {
            label {
                r#for: "username",
                "Username: "
            }
            input {
                id: "username",
                value: "{username}",
                oninput: move |evt| {
                    username.set(evt.value.clone())
                }
            }
            div { class: "w-100" }
            label {
                r#for: "password",
                "New Password: "
            }
            input {
                id: "password",
                value: "{password}",
                oninput: move |evt| {
                    password.set(evt.value.clone())
                }
            }
            div { class: "w-100" }
            button {
                class: "btn btn-outline-danger",
                onclick: move |_| cx.spawn({
                    to_owned![username, password, auth_token];
                    let display_message = cx.props.display_message.clone();

                    async move {
                        let client = reqwest::Client::new();

                        if !username.is_empty()
                        && !password.is_empty() {
                            let names = models::AdminResetPassword {
                                username: (*username.current()).clone(),
                                password: (*password.current()).clone(),
                            };

                            let res = client.post(api::RESET_PASSWORD.as_str())
                                .json(&names).bearer_auth(auth_token.unwrap_or("".into()))
                                .send().await
                                .handle_result::<()>(UIMessage::error("Resetting password failed.".to_string())).await;

                            match res {
                                Ok(()) => {
                                    display_message.send(UIMessage::info(format!("Updated password of {}.", username.current())));
                                }
                                Err(e) => display_message.send(e)
                            }
                        }
                    }
                }),
                "Reset Password"
            }
        }
    })
}

pub fn AdminPage<'a>(cx: Scope<'a, MessageProps<'a>>) -> Element<'a> {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);

    cx.render(rsx! {
        div {
            MergeNames { display_message: cx.props.display_message },
            ResetPassword { display_message: cx.props.display_message }
        }
    })
}
