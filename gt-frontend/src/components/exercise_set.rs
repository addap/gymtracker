#![allow(non_snake_case)]
use dioxus::prelude::*;

use gt_core::models;

#[inline_props]
pub fn ExerciseSetWeighted(cx: Scope, exs: models::ExerciseSetWeightedQuery) -> Element {
    //
    cx.render(rsx! {
        div {
            exs.name.clone()
            br {}
            p { "Weight" }
            p { exs.weight.to_string() }
            p { "Reps" }
            p { exs.reps.to_string() }
        }
    })
}

#[inline_props]
pub fn ExerciseSetBodyweight(cx: Scope, exs: models::ExerciseSetBodyweightQuery) -> Element {
    //
    cx.render(rsx! {
        div {
            exs.name.clone()
            br {}
            p { "Reps" }
            p { exs.reps.to_string() }
        }
    })
}

#[inline_props]
pub fn ExerciseSet(cx: Scope, exs: models::ExerciseSetQuery) -> Element {
    cx.render(match exs.clone() {
        models::ExerciseSetQuery::Weighted(exs) => rsx! { ExerciseSetWeighted { exs: exs } },
        models::ExerciseSetQuery::Bodyweight(exs) => rsx! { ExerciseSetBodyweight { exs: exs } },
    })
}
