#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::{use_read, use_atom_state, Atom};
use log::info;

use crate::{
    api,
    auth::ACTIVE_AUTH_TOKEN,
    messages::UIMessage,
    components as c,
    request_ext::RequestExt,
};
use gt_core::models;

static W_EXERCISE_SET_NAME: Atom<String> = |_| "".to_string();
static W_EXERCISE_SET_WEIGHT: Atom<f64> = |_| 1.0;
static B_EXERCISE_SET_NAME: Atom<String> = |_| "".to_string();

#[derive(Props)]
pub struct AddExerciseProps<'a> {
    exercise_names: Vec<models::ExerciseName>,
    fetch_names: &'a Coroutine<c::main_page::FetchNames>,
    display_message: &'a Coroutine<UIMessage>,
}

pub fn AddExerciseSetWeighted<'a>(cx: Scope<'a, AddExerciseProps<'a>>) -> Element<'a> {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);
    let w_exercise_set_name = use_atom_state(&cx, W_EXERCISE_SET_NAME);
    let w_exercise_set_weight = use_atom_state(&cx, W_EXERCISE_SET_WEIGHT);
    let w_exercise_set_reps = use_state(&cx, || 0);

    let names_datalist = cx.props.exercise_names.iter()
        .filter(|exn| exn.kind == models::ExerciseKind::Weighted)
        .map(|exn| 
            rsx! { 
                option { value: exn.name.as_str() }
            }
        );

    cx.render(rsx! {
        div {
            class: "bg-body-tertiary my-2 p-2",
            div {
                class: "row gap-1",
                p { 
                    class: "col-12",
                    "Weighted Exercise Set" 
                }
                div {
                    class: "form-group col-12 col-sm-auto",
                    label {
                        r#for: "w-exercise-names",
                        "Exercise Name"
                    }
                    input {
                        class: "form-control",
                        id: "w-exercise-names",
                        list: "w-exercise-names-list",
                        value: "{w_exercise_set_name}",
                        placeholder: "exercise name",
                        oninput: move |evt| w_exercise_set_name.set(evt.value.clone()),
                    }
                    datalist {
                        id: "w-exercise-names-list",
                        names_datalist
                    }
                }
                div {
                    class: "form-group col-12 col-sm-2",
                    label {
                        r#for: "w-exercise-set-weight",
                        "Weight (kg)"
                    }
                    input {
                        class: "form-control",
                        id: "w-exercise-set-weight",
                        r#type: "number",
                        value: "{w_exercise_set_weight}",
                        step: "any",
                        oninput: move |evt| {
                            if let Ok(v) = evt.value.parse() {
                                w_exercise_set_weight.set(v)
                            }
                        }
                    }
                }
                div {
                    class: "form-group col-12 col-sm-2",
                    label {
                        r#for: "w-exercise-set-reps",
                        "Reps"
                    }
                    input {
                        class: "form-control",
                        id: "w-exercise-set-reps",
                        r#type: "number",
                        min: "0",
                        value: "{w_exercise_set_reps}",
                        oninput: move |evt| {
                            if let Ok(v) = evt.value.parse() {
                                w_exercise_set_reps.set(v)
                            }
                        }
                    }
                }
                div { class: "w-100" }
                div {
                    button {
                        class: "col-3 col-sm-1 btn btn-sm btn-outline-success",
                        onclick: move |_| cx.spawn({
                            to_owned![w_exercise_set_name, w_exercise_set_reps, w_exercise_set_weight, auth_token];
                            let fetch_names = cx.props.fetch_names.clone();
                            let display_message = cx.props.display_message.clone();
                            
                            async move {
                                let client = reqwest::Client::new();
                                
                                if !w_exercise_set_name.current().is_empty()
                                && *w_exercise_set_reps.current() > 0 {
                                    let exs: models::ExerciseSet = (models::ExerciseSetWeighted {
                                        name: (*w_exercise_set_name.current()).clone(),
                                        reps: *w_exercise_set_reps.current(),
                                        weight: *w_exercise_set_weight.current(),
                                    }).into();

                                    let res = client.post(api::EXERCISE_SET.as_str())
                                        .json(&exs).bearer_auth(auth_token.unwrap_or("".into()))
                                        .send().await
                                        .handle_result(UIMessage::error("Submitting exercise failed.".to_string())).await;

                                    match res {
                                        Ok(()) => {
                                            fetch_names.send(c::main_page::FetchNames);
                                            display_message.send(UIMessage::info(format!("Added exercise \"{}\" x{} ({}kg)",
                                                w_exercise_set_name.current(),
                                                w_exercise_set_reps.current(),
                                                w_exercise_set_weight.current()
                                            )));

                                            // Reset reps so that you cannot accidentally submit it twice.
                                            w_exercise_set_reps.set(0);

                                        }
                                        Err(e) => display_message.send(e)
                                    }
                                }
                            }
                        }),
                        "+"
                    }
                }
            }
        }
    })
}

pub fn AddExerciseSetBodyweight<'a>(cx: Scope<'a, AddExerciseProps<'a>>) -> Element<'a> {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);
    let b_exercise_set_name = use_atom_state(&cx, B_EXERCISE_SET_NAME);
    let b_exercise_set_reps = use_state(&cx, || 0);

    let names_datalist = cx.props.exercise_names.iter()
        .filter(|exn| exn.kind == models::ExerciseKind::Bodyweight)
        .map(|exn| 
            rsx! { 
                option { value: exn.name.as_str() }
            }
        );

    cx.render(rsx! {
        div {
            class: "bg-body-tertiary my-2 p-2",
            div {
                class: "row gap-1",
                p { 
                    class: "col-12", 
                    "Bodyweight Exercise Set" 
                }
                div {
                    class: "form-group col-12 col-sm-auto",
                    label {
                        r#for: "b-exercise-names",
                        "Exercise Name"
                    }
                    input {
                        class: "form-control",
                        id: "b-exercise-names",
                        list: "b-exercise-names-list",
                        value: "{b_exercise_set_name}",
                        placeholder: "exercise name",
                        oninput: move |evt| b_exercise_set_name.set(evt.value.clone()),
                    }
                    datalist {
                        id: "b-exercise-names-list",
                        names_datalist
                    }
                }
                div {
                    class: "form-group col-12 col-sm-2",
                    label {
                        r#for: "b-exercise-set-reps",
                        "Reps"
                    }
                    input {
                        class: "form-control",
                        id: "b-exercise-set-reps",
                        r#type: "number",
                        min: "0",
                        value: "{b_exercise_set_reps}",
                        oninput: move |evt| {
                            if let Ok(v) = evt.value.parse() {
                                b_exercise_set_reps.set(v)
                            }
                        }
                    }
                }
                div { class: "w-100" }
                div {
                    button {
                        class: "col-3 col-sm-1 btn btn-sm btn-outline-success",
                        onclick: move |_| cx.spawn({
                            to_owned![b_exercise_set_name, b_exercise_set_reps, auth_token];
                            let fetch_names = cx.props.fetch_names.clone();
                            let display_message = cx.props.display_message.clone();
                            
                            async move {
                                let client = reqwest::Client::new();
                                
                                if !b_exercise_set_name.current().is_empty() 
                                && *b_exercise_set_reps.current() > 0 {
                                    let exs: models::ExerciseSet = (models::ExerciseSetBodyweight {
                                        name: (*b_exercise_set_name.current()).clone(),
                                        reps: *b_exercise_set_reps.current(),
                                    }).into();

                                    let res = client.post(api::EXERCISE_SET.as_str())
                                        .json(&exs).bearer_auth(auth_token.unwrap_or("".into()))
                                        .send().await
                                        .handle_result(UIMessage::error("Submitting exercise failed.".to_string())).await;

                                    match res {
                                        Ok(()) => {
                                            fetch_names.send(c::main_page::FetchNames);
                                            display_message.send(UIMessage::info(format!("Added exercise \"{}\" x {}",
                                                b_exercise_set_name.current(),
                                                b_exercise_set_reps.current()
                                            )));

                                            // Reset reps so that you cannot accidentally submit it twice.
                                            b_exercise_set_reps.set(0);
                                        }
                                        Err(e) => display_message.send(e)
                                    }
                                }
                            }
                        }),
                        "+"
                    }
                }
            }
        }
    })
}