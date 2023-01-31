#![allow(non_snake_case)]
use const_format::concatcp;
use dioxus::prelude::*;
use dioxus_router::Link;
use fermi::use_read;
use log::info;

use crate::components as c;
use crate::{
    api_url,
    auth::{is_logged_in, ACTIVE_AUTH_TOKEN},
    APP_BASE,
};
use gt_core::models;

fn LoggedInMainPage(cx: Scope) -> Element {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);

    let fetch = use_future(&cx, (), |()| {
        to_owned![auth_token];

        async move {
            let client = reqwest::Client::new();
            let res = client
                .get(api_url("/exercise/name"))
                .bearer_auth(auth_token.unwrap_or("".into()))
                .send()
                .await;

            if let Err(ref e) = res {
                info!("{}", e);
                return vec![];
            }
            let exercise_names = res
                .unwrap()
                .json::<Vec<models::ExerciseName>>()
                .await
                .unwrap();

            exercise_names
        }
    });

    let exercise_names = match fetch.value() {
        Some(v) => v.clone(),
        None => vec![],
    };

    cx.render(rsx! {
        div {
            c::AddExerciseSetWeighted { exercise_names: exercise_names.clone() }
            br {}
            c::AddExerciseSetBodyweight { exercise_names: exercise_names.clone() }
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
