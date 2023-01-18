use axum::{extract::State, Extension, Json};
use chrono::Utc;
use sea_orm::*;

use crate::{AppState, Result};
use gt_core::entities::{prelude::*, *};
use gt_core::models;

pub async fn add_exercise_name(
    State(state): State<AppState>,
    #[allow(unused_variables)] Extension(user): Extension<user_login::Model>,
    Json(payload): Json<models::AddExercise>,
) -> Result<Json<()>> {
    let exercise_name = exercise_name::ActiveModel {
        name: ActiveValue::Set(payload.name),
        ..Default::default()
    };

    ExerciseName::insert(exercise_name)
        .exec(&state.conn)
        .await?;

    Ok(Json(()))
}

pub async fn get_all_exercise_names(
    State(state): State<AppState>,
    #[allow(unused_variables)] Extension(user): Extension<user_login::Model>,
) -> Result<Json<Vec<exercise_name::Model>>> {
    let res = ExerciseName::find().all(&state.conn).await?;

    Ok(Json(res))
}

pub async fn add_exercise_set_for_user(
    State(state): State<AppState>,
    Extension(user): Extension<user_login::Model>,
    Json(payload): Json<models::ExerciseSet>,
) -> Result<Json<()>> {
    let new_exercise_set = exercise_set::ActiveModel {
        user_id: ActiveValue::Set(user.id),
        name: ActiveValue::Set(payload.name),
        reps: ActiveValue::Set(payload.reps),
        weight: ActiveValue::Set(payload.weight),
        created_at: ActiveValue::Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    ExerciseSet::insert(new_exercise_set)
        .exec(&state.conn)
        .await?;

    Ok(Json(()))
}

pub async fn get_exercise_sets_for_user(
    State(state): State<AppState>,
    Extension(user): Extension<user_login::Model>,
) -> Result<Json<Vec<exercise_set::Model>>> {
    let res = user.find_related(ExerciseSet).all(&state.conn).await?;

    Ok(Json(res))
}
