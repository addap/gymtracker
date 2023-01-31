#![allow(non_snake_case)]
use chrono::{Local, TimeZone};
use dioxus::prelude::*;
use log::info;

use gt_core::models;

#[inline_props]
pub fn ExerciseSetWeighted<'a>(
    cx: Scope,
    exs: &'a models::ExerciseSetWeightedQuery,
) -> Element<'a> {
    //
    // TODO move to global initialization
    let created_at_local = Local.from_utc_datetime(&exs.created_at);

    cx.render(rsx! {
        div {
            exs.name.clone()
            br {}
            div {
                format!("Weight: {}kg", exs.weight.to_string())
            }
            div {
                format!("Reps: {}", exs.reps.to_string())
            }
            div {
                if created_at_local.date_naive() == Local::now().date_naive() {
                    format!("At {}", created_at_local.time().format("%H:%M:%S").to_string())
                } else {
                    format!("On {}", created_at_local.date_naive().to_string())
                }
            }
        }
    })
}

#[inline_props]
pub fn ExerciseSetBodyweight<'a>(
    cx: Scope,
    exs: &'a models::ExerciseSetBodyweightQuery,
) -> Element<'a> {
    //
    cx.render(rsx! {
        div {
            exs.name.clone()
            br {}
            div {
                "Reps: "
                exs.reps.to_string()
            }
        }
    })
}

#[inline_props]
pub fn ExerciseSet<'a>(cx: Scope, exs: &'a models::ExerciseSetQuery) -> Element<'a> {
    cx.render(match exs.clone() {
        models::ExerciseSetQuery::Weighted(exs) => rsx! { ExerciseSetWeighted { exs: exs } },
        models::ExerciseSetQuery::Bodyweight(exs) => rsx! { ExerciseSetBodyweight { exs: exs } },
    })
}
