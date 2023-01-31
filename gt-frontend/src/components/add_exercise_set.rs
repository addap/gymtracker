#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::use_read;
use log::info;

use crate::{
    api_url, auth::ACTIVE_AUTH_TOKEN
};
use crate::components as c;
use gt_core::models;


#[derive(Props)]
pub struct AddExerciseProps<'a> {
    exercise_names: Vec<models::ExerciseName>,
    fetch_names: &'a Coroutine<c::main_page::FetchNames>,
}

pub fn AddExerciseSetWeighted<'a>(cx: Scope<'a, AddExerciseProps<'a>>) -> Element<'a> {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);
    let w_exercise_set_name = use_state(&cx, || "".to_string());
    let w_exercise_set_weight = use_state(&cx, || 1.0);
    let w_exercise_set_reps = use_state(&cx, || 1);



    let names_datalist = cx.props.exercise_names.iter()
        .filter(|exn| exn.kind == models::ExerciseKind::Weighted)
        .map(|exn| 
            rsx! { 
                option { value: exn.name.as_str() }
            }
        );

    cx.render(rsx! {
        input {
            list: "w-exercise-names",
            value: "{w_exercise_set_name}",
            oninput: move |evt| w_exercise_set_name.set(evt.value.clone()),
        }
        datalist {
            id: "w-exercise-names",
            names_datalist
        }
        br {}
        input {
            r#type: "number",
            value: "{w_exercise_set_weight}",
            step: "any",
            oninput: move |evt| {
                if let Ok(v) = evt.value.parse() {
                    w_exercise_set_weight.set(v)
                }
            }
        }
        br {}
        input {
            r#type: "number",
            min: "1",
            value: "{w_exercise_set_reps}",
            oninput: move |evt| {
                if let Ok(v) = evt.value.parse() {
                    w_exercise_set_reps.set(v)
                }
            }
        }
        br {}
        button {
            onclick: move |_| cx.spawn({
                to_owned![w_exercise_set_name, w_exercise_set_reps, w_exercise_set_weight, auth_token];
                let fetch_names = cx.props.fetch_names.clone();
                
                async move {
                    let client = reqwest::Client::new();
                    
                    if !w_exercise_set_name.current().is_empty() {
                        let exs: models::ExerciseSet = (models::ExerciseSetWeighted {
                            name: (*w_exercise_set_name.current()).clone(),
                            reps: *w_exercise_set_reps.current(),
                            weight: *w_exercise_set_weight.current(),
                        }).into();

                        let res = client.post(api_url("/exercise/set")).json(&exs).bearer_auth(auth_token.unwrap_or("".into()))
                            .send().await;
                        if let Err(ref e) = res {
                            info!("{}", e);
                            return;
                        }

                        fetch_names.send(c::main_page::FetchNames);
                    }
                }
            }),
            "+"
        }
    })
}

pub fn AddExerciseSetBodyweight<'a>(cx: Scope<'a, AddExerciseProps<'a>>) -> Element<'a> {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);
    let w_exercise_set_name = use_state(&cx, || "".to_string());
    let w_exercise_set_reps = use_state(&cx, || 1);

    let names_datalist = cx.props.exercise_names.iter()
        .filter(|exn| exn.kind == models::ExerciseKind::Bodyweight)
        .map(|exn| 
            rsx! { 
                option { value: exn.name.as_str() }
            }
        );

    cx.render(rsx! {
        input {
            list: "b-exercise-names",
            value: "{w_exercise_set_name}",
            oninput: move |evt| w_exercise_set_name.set(evt.value.clone()),
        }
        datalist {
            id: "b-exercise-names",
            names_datalist
        }
        br {}
        input {
            r#type: "number",
            min: "1",
            value: "{w_exercise_set_reps}",
            oninput: move |evt| {
                if let Ok(v) = evt.value.parse() {
                    w_exercise_set_reps.set(v)
                }
            }
        }
        br {}
        button {
            onclick: move |_| cx.spawn({
                to_owned![w_exercise_set_name, w_exercise_set_reps, auth_token];
                let fetch_names = cx.props.fetch_names.clone();
                
                async move {
                    let client = reqwest::Client::new();
                    
                    if !w_exercise_set_name.current().is_empty() {
                        let exs: models::ExerciseSet = (models::ExerciseSetBodyweight {
                            name: (*w_exercise_set_name.current()).clone(),
                            reps: *w_exercise_set_reps.current(),
                        }).into();

                        let res = client.post(api_url("/exercise/set")).json(&exs).bearer_auth(auth_token.unwrap_or("".into()))
                            .send().await;
                        if let Err(ref e) = res {
                            info!("{}", e);
                            return;
                        }

                        fetch_names.send(c::main_page::FetchNames);
                    }
                }
            }),
            "+"
        }
    })
}