#![allow(non_snake_case)]
use chrono::{DateTime, FixedOffset, Local};
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
    let now = js_sys::Date::new_0();
    info!("jsnow {:?}", now);
    let offset_min = now.get_timezone_offset();
    info!("offset minutes {:?}", offset_min);
    let offset = FixedOffset::east_opt(-(offset_min * 60.0) as i32).unwrap();

    let created_at_aware: DateTime<FixedOffset> = DateTime::from_utc(exs.created_at, offset);
    info!("created at utc {}", exs.created_at);
    info!("created at aware {}", created_at_aware);
    info!("now {}", Local::now().date_naive());

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
                if created_at_aware.date_naive() == Local::now().date_naive() {
                    format!("At {}", created_at_aware.time().format("%H:%M:%S").to_string())
                } else {
                    format!("On {}", created_at_aware.date_naive().to_string())
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
