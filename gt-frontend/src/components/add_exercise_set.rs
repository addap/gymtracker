use dioxus::prelude::*;

pub fn AddExerciseSet(cx: Scope) -> Element {
    let exercise_set_name = use_state(&cx, || "");
    let exercise_set_weight = use_state(&cx, || 1.0);
    let exercise_set_reps = use_state(&cx, || 1);

    cx.render(rsx! {
        input {
            r#type: "text",
            id: "exercise-set-name",
            name: "exercise-set-name",
        }
        input {
            r#type: "text",
            id: "exercise-set-weight",
            name: "exercise-set-weight",
        }
        input {
            r#type: "number",
            id: "exercise-set-reps",
            name: "exercise-set-reps",
            min: "1",
            max: "100"
        }
        input {
            onclick: move |_| {},
            r#type: "button",
            id: "add_set",
            name: "add_set",
            value: "+"
        }
    })
}
