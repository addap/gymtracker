use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AddExercise {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct ExerciseSet {
    pub name: String,
    pub reps: i32,
    pub weight: f32,
}
