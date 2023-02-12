#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::use_read;
use futures_util::StreamExt;
use log::info;

use crate::components as c;
use crate::request_ext::RequestExt;
use crate::{
    api,
    auth::{is_logged_in, ACTIVE_AUTH_TOKEN},
    messages::MessageProps,
    UIMessage,
};
use gt_core::models;

#[derive(Debug, Clone, Copy)]
pub struct FetchNames;

fn LoggedInMainPage<'a>(cx: Scope<'a, MessageProps<'a>>) -> Element<'a> {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);
    let exercise_names = use_state(&cx, || vec![]);

    let fetch_names = use_coroutine(&cx, |mut rx: UnboundedReceiver<FetchNames>| {
        to_owned![auth_token, exercise_names];
        let empty_auth_token: models::AuthToken = "".into();
        let display_message = cx.props.display_message.clone();

        async move {
            while let Some(FetchNames) = rx.next().await {
                let token: &models::AuthToken = auth_token.as_ref().unwrap_or(&empty_auth_token);
                let client = reqwest::Client::new();
                let res = client
                    .get(api::EXERCISE_NAME.as_str())
                    .bearer_auth(token)
                    .send()
                    .await
                    .handle_result(UIMessage::error(
                        "Fetching exercise names failed.".to_string(),
                    ))
                    .await;

                match res {
                    Ok(names) => exercise_names.set(names),
                    Err(e) => display_message.send(e),
                }
            }
        }
    });
    use_future(&cx, (), |()| {
        to_owned![fetch_names];
        async move { fetch_names.send(FetchNames) }
    });

    cx.render(rsx! {
        div {
            c::AddExerciseSetWeighted {
                exercise_names: exercise_names.get().to_owned(),
                fetch_names: fetch_names,
                display_message: &cx.props.display_message
            }
            c::AddExerciseSetBodyweight {
                exercise_names: exercise_names.get().to_owned(),
                fetch_names: fetch_names,
                display_message: &cx.props.display_message
            }
        }
    })
}
fn LoggedOutMainPage(cx: Scope) -> Element {
    None
}

pub fn MainPage<'a>(cx: Scope<'a, MessageProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            div {
                class: "row",
                div {
                    p {
                        class: "col",
                        "Main page"
                    }
                }
            }
            if is_logged_in(&cx) {
                rsx!{ LoggedInMainPage { display_message: &cx.props.display_message } }
            }
        }
    })
}
