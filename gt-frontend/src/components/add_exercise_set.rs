#![allow(non_snake_case)]
use chrono::{Local, NaiveDateTime, TimeZone};
use dioxus::prelude::*;
use derive_more::Deref;
use fermi::{use_read, use_atom_state, Atom};

use crate::{
    api,
    auth::ACTIVE_AUTH_TOKEN,
    messages::UIMessage,
    components as c,
    request_ext::RequestExt,
};
use gt_core::models;

/// Wrapper types to prevent merging of Atoms.
/// c.f. https://github.com/DioxusLabs/dioxus/issues/706
#[derive(Deref)]
struct Wrapper1<T>(T);
#[derive(Deref)]
struct Wrapper2<T>(T);

static W_EXERCISE_SET_NAME: Atom<Wrapper1<String>> = |_| Wrapper1("".to_string());
static W_EXERCISE_SET_WEIGHT: Atom<f64> = |_| 1.0;
static B_EXERCISE_SET_NAME: Atom<Wrapper2<String>> = |_| Wrapper2("".to_string());

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
    let w_exercise_set_date = use_state(&cx, || Local::now().naive_local().format("%Y-%m-%dT%H:%M").to_string());

    let names_datalist = cx.props.exercise_names.iter()
        .filter(|exn| exn.kind == models::ExerciseKind::Weighted)
        .map(|exn| 
            rsx! { 
                option { value: exn.name.as_str() }
            }
        );

    cx.render(rsx! {
        div {
            // my-3: vertical margin 3
            // p-2: h&v padding 2
            class: "bg-body-tertiary my-3 p-2",
            form {
                // g-1: gutters 1
                // g-sm-2: gutters at the small breakpoint 2
                class: "row g-1 g-sm-2",
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
                        value: "{w_exercise_set_name.0}",
                        placeholder: "exercise name",
                        autocomplete: "off",
                        oninput: move |evt| w_exercise_set_name.set(Wrapper1(evt.value.clone())),
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
                    class: "form-group col col-sm-auto",
                    label {
                        r#for: "w-ecercise-set-date",
                        "Date"
                    }
                    input {
                        class: "form-control",
                        id: "w-exercise-set-date",
                        r#type: "datetime-local",
                        value: "{w_exercise_set_date}",
                        oninput: move |evt| {
                            w_exercise_set_date.set(evt.value.clone())
                        }
                    }
                }
                div {
                    class: "col-auto d-flex align-items-end",
                    button {
                        r#type: "button",
                        class: "btn btn-outline-info",
                        onclick: move |_| {
                            w_exercise_set_date.set(Local::now().naive_local().format("%Y-%m-%dT%H:%M").to_string())
                        },
                        "🕒"
                    }
                }
                div { class: "w-100" }
                div {
                    button {
                        r#type: "button",
                        class: "col-3 col-sm-1 btn btn-sm btn-outline-success",
                        onclick: move |_| cx.spawn({
                            to_owned![w_exercise_set_name, w_exercise_set_reps, w_exercise_set_weight, w_exercise_set_date, auth_token];
                            let fetch_names = cx.props.fetch_names.clone();
                            let display_message = cx.props.display_message.clone();
                            
                            async move {
                                let client = reqwest::Client::new();
                                
                                if !w_exercise_set_name.current().is_empty()
                                && *w_exercise_set_reps.current() > 0 {
                                    // convert the datetime-local into a utc datetime string
                                    let created_at = NaiveDateTime::parse_from_str(w_exercise_set_date.current().as_str(), "%Y-%m-%dT%H:%M").unwrap();
                                    let created_at = Local.from_local_datetime(&created_at).unwrap();
    

                                    let exs: models::ExerciseSet = (models::ExerciseSetWeighted {
                                        name: w_exercise_set_name.current().0.clone(),
                                        reps: *w_exercise_set_reps.current(),
                                        weight: *w_exercise_set_weight.current(),
                                        created_at: created_at.naive_utc().format("%Y-%m-%dT%H:%M").to_string(),
                                    }).into();

                                    let res = client.post(api::EXERCISE_SET.as_str())
                                        .json(&exs).bearer_auth(auth_token.unwrap_or("".into()))
                                        .send().await
                                        .handle_result(UIMessage::error("Submitting exercise failed.".to_string())).await;

                                    match res {
                                        Ok(()) => {
                                            fetch_names.send(c::main_page::FetchNames);
                                            display_message.send(UIMessage::info(format!("Added exercise \"{}\" x{} ({}kg)",
                                                w_exercise_set_name.current().0,
                                                *w_exercise_set_reps.current(),
                                                *w_exercise_set_weight.current()
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
    let b_exercise_set_date = use_state(&cx, || Local::now().naive_local().format("%Y-%m-%dT%H:%M").to_string());

    let names_datalist = cx.props.exercise_names.iter()
        .filter(|exn| exn.kind == models::ExerciseKind::Bodyweight)
        .map(|exn| 
            rsx! { 
                option { value: exn.name.as_str() }
            }
        );

    cx.render(rsx! {
        div {
            class: "bg-body-tertiary my-3 p-2",
            form {
                class: "row g-1 g-sm-2",
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
                        value: "{b_exercise_set_name.0}",
                        placeholder: "exercise name",
                        autocomplete: "off",
                        oninput: move |evt| b_exercise_set_name.set(Wrapper2(evt.value.clone())),
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
                    class: "form-group col col-sm-auto",
                    label {
                        r#for: "b-ecercise-set-date",
                        "Date"
                    }
                    input {
                        class: "form-control",
                        id: "b-exercise-set-date",
                        r#type: "datetime-local",
                        value: "{b_exercise_set_date}",
                        oninput: move |evt| {
                            b_exercise_set_date.set(evt.value.clone())
                        }
                    }
                }
                div {
                    class: "col-auto d-flex align-items-end",
                    button {
                        r#type: "button",
                        class: "btn btn-outline-info",
                        onclick: move |_| {
                            b_exercise_set_date.set(Local::now().naive_local().format("%Y-%m-%dT%H:%M").to_string())
                        },
                        "🕒"
                    }
                }
                div { class: "w-100" }
                div {
                    button {
                        r#type: "button",
                        class: "col-3 col-sm-1 btn btn-sm btn-outline-success",
                        onclick: move |_| cx.spawn({
                            to_owned![b_exercise_set_name, b_exercise_set_reps, b_exercise_set_date, auth_token];
                            let fetch_names = cx.props.fetch_names.clone();
                            let display_message = cx.props.display_message.clone();
                            
                            async move {
                                let client = reqwest::Client::new();
                                
                                if !b_exercise_set_name.current().is_empty() 
                                && *b_exercise_set_reps.current() > 0 {
                                    // convert the datetime-local into a utc datetime string
                                    let created_at = NaiveDateTime::parse_from_str(b_exercise_set_date.current().as_str(), "%Y-%m-%dT%H:%M").unwrap();
                                    let created_at = Local.from_local_datetime(&created_at).unwrap();

                                    let exs: models::ExerciseSet = (models::ExerciseSetBodyweight {
                                        name: b_exercise_set_name.current().0.clone(),
                                        reps: *b_exercise_set_reps.current(),
                                        created_at: created_at.naive_utc().format("%Y-%m-%dT%H:%M").to_string(),
                                    }).into();

                                    let res = client.post(api::EXERCISE_SET.as_str())
                                        .json(&exs).bearer_auth(auth_token.unwrap_or("".into()))
                                        .send().await
                                        .handle_result(UIMessage::error("Submitting exercise failed.".to_string())).await;

                                    match res {
                                        Ok(()) => {
                                            fetch_names.send(c::main_page::FetchNames);
                                            display_message.send(UIMessage::info(format!("Added exercise \"{}\" x {}",
                                                b_exercise_set_name.current().0,
                                                *b_exercise_set_reps.current()
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