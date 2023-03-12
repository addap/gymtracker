#![allow(non_snake_case)]
use chrono::{Local, TimeZone};
use dioxus::prelude::*;
use fermi::use_read;

use crate::{
    auth::ACTIVE_AUTH_TOKEN,
    request_ext::RequestExt,
    api,
    messages::UIMessage,
    util::*,
};
use gt_core::models;

#[inline_props]
pub fn ExerciseSetWeighted<'a>(
    cx: Scope,
    exs: &'a models::ExerciseSetWeightedQuery,
) -> Element<'a> {
    let created_at_local = Local.from_utc_datetime(&exs.created_at);

    cx.render(rsx! {
        div {
            class: "row",
            div {
                class: "col",
                p { class: "fw-bold",
                    exs.name.clone() }
            }
            div {
                class: "col-auto",
                p { class: "fw-bold",
                    format_weighted_reps(exs.reps, exs.weight) }
            } 
            div { class: "w-100" }
            div {
                class: "col"
            }
            div {
                class: "col-auto",
                p { class: "fw-light",
                    format_date(created_at_local) }
            }
        }
    })
}

#[inline_props]
pub fn ExerciseSetBodyweight<'a>(
    cx: Scope,
    exs: &'a models::ExerciseSetBodyweightQuery,
) -> Element<'a> {
    let created_at_local = Local.from_utc_datetime(&exs.created_at);

    cx.render(rsx! {
        div {
            class: "row",
            div {
                class: "col",
                p { class: "fw-bold",
                    exs.name.clone() }
            }
            div {
                class: "col-auto",
                p { class: "fw-bold",
                    format_bodyweight_reps(exs.reps) }
            }
            div { class: "w-100" }
            div {
                class: "col"
            }
            div {
                class: "col-auto",
                p { class: "fw-light",
                    format_date(created_at_local) }
            }
        }
    })
}

#[derive(Props)]
pub struct ExerciseSetProps<'a> {
    pub exs: &'a models::ExerciseSetQuery,
    pub display_message: &'a Coroutine<UIMessage>,
}

pub fn ExerciseSet<'a>(cx: Scope<'a, ExerciseSetProps<'a>>) -> Element<'a> {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);
    let deleted = use_state(&cx, || false);

    let info = match cx.props.exs {
        models::ExerciseSetQuery::Weighted(exs) => rsx! { ExerciseSetWeighted { exs: exs } },
        models::ExerciseSetQuery::Bodyweight(exs) => rsx! { ExerciseSetBodyweight { exs: exs } },
    };
    
    let exercise_set_id = cx.props.exs.id();
    let should_display = if *deleted.current() { "none" } else { "block" };

    cx.render(rsx! {
        li {
            class: "list-group-item",
            display: should_display,
            div {
                class: "row",
                div {
                    class: "col",
                    info
                }
                div {
                    class: "col-auto d-flex align-items-center",
                    button {
                        class: "btn btn-sm btn-outline-danger",
                        onclick: move |_| cx.spawn({
                            to_owned![auth_token, deleted];
                            let display_message = cx.props.display_message.clone();

                            async move {
                                let client = reqwest::Client::new();

                                let exs = models::ExerciseSetDelete { id: exercise_set_id };
                                
                                let res = client.delete(api::EXERCISE_SET.as_str())
                                    .json(&exs).bearer_auth(auth_token.unwrap_or("".into()))
                                    .send().await
                                    .handle_result::<()>(UIMessage::error("Deleting exercise failed.".to_string())).await;

                                match res {
                                    Ok(()) => deleted.set(true),
                                    Err(e) => display_message.send(e)
                                }
                            }
                        }),
                        "🗑️"
                    }
                }
            }
        }
    })
}
