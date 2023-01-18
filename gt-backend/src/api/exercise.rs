use axum::{extract::State, Json};
use sea_orm::*;
use serde::Deserialize;
use serde_json::{json, to_value, Value};

use crate::{AppState, Result};
use gt_core::entities::{prelude::*, *};

#[derive(Deserialize)]
pub struct AddExercise {
    name: String,
}

pub async fn add_exercise_name(
    State(state): State<AppState>,
    Json(payload): Json<AddExercise>,
    // ) -> Result<InsertResult<exercise_name::ActiveModel>, DbErr> {
) -> Result<Json<()>> {
    // TODO how to avoid unwrap and do error handling in axum?
    let exercise_name = exercise_name::ActiveModel {
        name: ActiveValue::Set(payload.name),
        ..Default::default()
    };
    let _ = ExerciseName::insert(exercise_name)
        .exec(&state.conn)
        .await?;

    Ok(Json(()))
}

pub async fn get_all_exercise_names(
    State(state): State<AppState>,
) -> Result<Json<Vec<exercise_name::Model>>> {
    let res: Vec<exercise_name::Model> = ExerciseName::find().all(&state.conn).await?;

    Ok(Json(res))
}

pub async fn get_exercise_sets_for_user(
    State(state): State<AppState>,
) -> Result<Json<Vec<exercise_set::Model>>> {
    todo!();
    Ok(Json(vec![]))
}

pub async fn add_exercise_set_for_user(State(state): State<AppState>) -> Result<Json<()>> {
    todo!();
    Ok(Json(()))
}
