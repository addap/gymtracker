// main.rs

use dioxus::prelude::*;
use gt_core::entities::exercise_name;
use serde_json::to_string_pretty;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let x = exercise_name::Model {
        id: 0,
        name: String::from("asd"),
    };
    let y = to_string_pretty(&x).unwrap();

    cx.render(rsx! {
        div {
            class: "container",
            y
        }
    })
}
