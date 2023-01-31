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
    APP_BASE,
};
use gt_core::models;

#[derive(Debug, Clone, Copy)]
pub struct FetchNames;

fn LoggedInMainPage(cx: Scope) -> Element {
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

    cx.render(rsx! {
        div {
            c::AddExerciseSetWeighted {
                exercise_names: exercise_names.get().to_owned(),
                fetch_names: fetch_names
            }
            br {}
            c::AddExerciseSetBodyweight {
                exercise_names: exercise_names.get().to_owned(),
                fetch_names: fetch_names
            }
        }
    })
}
fn LoggedOutMainPage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            Link { to: concatcp!(APP_BASE, "/register"), "Register" }
            br {}
            Link { to: concatcp!(APP_BASE, "/login"), "Login" }
        }
    })
}

pub fn MainPage(cx: Scope) -> Element {
    cx.render(rsx! {
        p { "Main page" }
        if is_logged_in(&cx) {
            rsx!{ LoggedInMainPage {} }
        } else {
            rsx!{ LoggedOutMainPage {} }
        }
    })
}
