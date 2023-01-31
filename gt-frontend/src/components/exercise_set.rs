#![allow(non_snake_case)]
use chrono::{DateTime, Local, TimeZone};
use dioxus::prelude::*;
use log::info;

use gt_core::models;

fn format_date(t: DateTime<Local>) -> String {
    if t.date_naive() == Local::now().date_naive() {
        format!("At {}", t.time().format("%H:%M:%S").to_string())
    } else {
        format!("On {}", t.date_naive().to_string())
    }
}

#[inline_props]
pub fn ExerciseSetWeighted<'a>(
    cx: Scope,
    exs: &'a models::ExerciseSetWeightedQuery,
) -> Element<'a> {
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
                format_date(created_at_local)
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
            exs.name.clone()
            br {}
            div {
                format!("Reps: {}", exs.reps.to_string())
            }
            div {
                format_date(created_at_local)
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
