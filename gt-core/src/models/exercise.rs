#[cfg(not(target_arch = "wasm32"))]
use sea_orm::FromQueryResult;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AddExercise {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct ExerciseSet {
    pub name: String,
    pub reps: i32,
    pub weight: f64,
}

#[derive(Deserialize, Serialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(FromQueryResult))]
pub struct ExerciseSetQuery {
    id: i32,
    user_id: i32,
    name_id: i32,
    name: String,
    reps: i32,
    weight: f64,
    created_at: chrono::NaiveDateTime,
}
