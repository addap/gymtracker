#![allow(non_snake_case)]
use const_format::concatcp;
use dioxus::prelude::*;
use fermi::use_read;
use gt_core::models::{ExerciseSet, ExerciseSetWeighted};
use log::info;

use crate::{
    api_url, auth::ActiveAuthToken
};

pub fn AddExerciseSetWeighted(cx: Scope) -> Element {
    let exercise_set_name = use_state(&cx, || "".to_string());
    let exercise_set_weight = use_state(&cx, || 1.0);
    let exercise_set_reps = use_state(&cx, || 1);
    let auth_token = use_read(&cx, ActiveAuthToken);

    cx.render(rsx! {
        input {
            r#type: "text",
            id: "exercise-set-name",
            name: "exercise-set-name",
            value: "{exercise_set_name}",
            oninput: move |evt| exercise_set_name.set(evt.value.clone()),
        }
        br {}
        input {
            r#type: "number",
            id: "exercise-set-weight",
            name: "exercise-set-weight",
            value: "{exercise_set_weight}",
            step: "any",
            oninput: move |evt| {
                if let Ok(v) = evt.value.parse() {
                    exercise_set_weight.set(v)
                }
            }
        }
        br {}
        input {
            r#type: "number",
            id: "exercise-set-reps",
            name: "exercise-set-reps",
            min: "1",
            value: "{exercise_set_reps}",
            oninput: move |evt| {
                if let Ok(v) = evt.value.parse() {
                    exercise_set_reps.set(v)
                }
            }
        }
        br {}
        input {
            onclick: move |_| cx.spawn({
                to_owned![exercise_set_name, exercise_set_reps, exercise_set_weight, auth_token];
                
                async move {
                    let client = reqwest::Client::new();
                    
                    if !exercise_set_name.current().is_empty() {
                        let exs: ExerciseSet = (ExerciseSetWeighted {
                            name: (*exercise_set_name.current()).clone(),
                            reps: *exercise_set_reps.current(),
                            weight: *exercise_set_weight.current(),
                        }).into();

                        let res = client.post(api_url("/exercise/set")).json(&exs).bearer_auth(auth_token.unwrap_or("".into()))
                        .send().await;
                        if let Err(ref e) = res {
                            info!("{}", e);
                            return;
                        }
                    }
                }
            }),
            r#type: "button",
            id: "add_set",
            name: "add_set",
            value: "+"
        }
    })
}
