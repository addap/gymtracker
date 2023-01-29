#![allow(non_snake_case)]
use const_format::concatcp;
use dioxus::prelude::*;
use fermi::use_read;
use gt_core::models::{self, exercise};
use log::info;

use crate::components as c;
use crate::{api_url, auth::ActiveAuthToken};

pub fn HistoryPage(cx: Scope) -> Element {
    // TODO maybe use use_ref?
    let exercise_history = use_state(&cx, || vec![]);
    let auth_token = use_read(&cx, ActiveAuthToken);

    let fetch = use_future(&cx, (), |()| {
        to_owned![exercise_history, auth_token];

        async move {
            let client = reqwest::Client::new();
            let res = client
                .get(api_url("/exercise/set"))
                .bearer_auth(auth_token.unwrap_or("".into()))
                .send()
                .await;

            if let Err(ref e) = res {
                info!("{}", e);
                return;
            }
            let history = res
                .unwrap()
                .json::<Vec<models::ExerciseSetQuery>>()
                .await
                .unwrap();

            exercise_history.set(history);
        }
    });

    let hlist = (*exercise_history.current())
        .clone()
        .into_iter()
        .map(|exs| {
            rsx! {
                li { c::ExerciseSet { exs: exs } }
            }
        });
    cx.render(rsx! {
        div {
            p { "History page" }
            if !exercise_history.current().is_empty() {
                rsx!{
                    ul { hlist
                    }
                }
            } else {
                rsx!{ p { "loading" } }
            }

        }
    })
}
