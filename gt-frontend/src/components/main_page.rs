#![allow(non_snake_case)]
use const_format::concatcp;
use dioxus::prelude::*;
use dioxus_router::Link;
use fermi::use_read;
use futures_util::StreamExt;
use log::info;

use crate::components as c;
use crate::{
    api_url,
    auth::{is_logged_in, ACTIVE_AUTH_TOKEN},
    messages::MessageProps,
    UIMessage, APP_BASE,
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

        async move {
            while let Some(FetchNames) = rx.next().await {
                let token: &models::AuthToken = auth_token.as_ref().unwrap_or(&empty_auth_token);
                let client = reqwest::Client::new();
                let res = client
                    .get(api_url("/exercise/name"))
                    .bearer_auth(token)
                    .send()
                    .await;

                if let Err(ref e) = res {
                    info!("{}", e);
                    return;
                }
                let names = res
                    .unwrap()
                    .json::<Vec<models::ExerciseName>>()
                    .await
                    .unwrap();

                exercise_names.set(names);
            }
        }
    });
    use_future(&cx, (), |()| {
        to_owned![fetch_names];
        async move { fetch_names.send(FetchNames) }
    });

    cx.render(rsx! {
        div {
            class: "row",
            c::AddExerciseSetWeighted {
                exercise_names: exercise_names.get().to_owned(),
                fetch_names: fetch_names,
                display_message: &cx.props.display_message
            }
            br {}
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
