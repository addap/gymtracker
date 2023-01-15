use axum::{extract::State, Json};
use sea_orm::*;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::AppState;
use gt_core::entities::{prelude::*, *};

#[derive(Deserialize)]
pub struct AddExercise {
    name: String,
}

pub async fn add_exercise_name(
    State(state): State<AppState>,
    Json(payload): Json<AddExercise>,
    // ) -> Result<InsertResult<exercise_name::ActiveModel>, DbErr> {
) -> Json<Value> {
    // TODO how to avoid unwrap and do error handling in axum?
    let exercise_name = exercise_name::ActiveModel {
        name: ActiveValue::Set(payload.name),
        ..Default::default()
    };
    let res = ExerciseName::insert(exercise_name)
        .exec(&state.conn)
        .await
        .unwrap();

    Json(json!(()))
}

pub async fn get_all_exercise_names(State(state): State<AppState>) -> Json<Value> {
    let res = ExerciseName::find()
        .all(&state.conn)
        .await
        .unwrap()
        .into_iter()
        .map(|exercise_name| exercise_name.name)
        .collect::<Vec<_>>();

    Json(json!(res))
}
