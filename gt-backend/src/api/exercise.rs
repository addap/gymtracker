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
    // get or create exercise name
    let name = ExerciseName::find()
        .filter(exercise_name::Column::Name.eq(payload.name.clone()))
        .one(&state.conn)
        .await?;

    let name_id = if let Some(name) = name {
        name.id
    } else {
        let new_name = exercise_name::ActiveModel {
            name: ActiveValue::Set(payload.name),
            ..Default::default()
        };
        let res = ExerciseName::insert(new_name).exec(&state.conn).await?;
        res.last_insert_id
    };

    let new_exercise_set = exercise_set::ActiveModel {
        user_id: ActiveValue::Set(user.id),
        name_id: ActiveValue::Set(name_id),
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
) -> Result<Json<Vec<models::ExerciseSetQuery>>> {
    let q = ExerciseSet::find()
        .filter(exercise_set::Column::UserId.eq(user.id))
        .column_as(exercise_name::Column::Name, "name")
        .join(
            JoinType::InnerJoin,
            exercise_set::Relation::ExerciseName.def(),
        );

    log::info!("{}", q.build(DbBackend::Sqlite).to_string());

    let res = q
        .into_model::<models::ExerciseSetQuery>()
        .all(&state.conn)
        .await?;

    Ok(Json(res))
}
