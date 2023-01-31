use axum::{extract::State, Extension, Json};
use chrono::Utc;
use sea_orm::*;

use crate::{AppState, Result};
use gt_core::entities::{prelude::*, *};
use gt_core::models;

pub async fn add_exercise_name(
    State(state): State<AppState>,
    #[allow(unused_variables)] Extension(user): Extension<user_login::Model>,
    Json(payload): Json<models::ExerciseName>,
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
) -> Result<Json<Vec<models::ExerciseName>>> {
    let res = ExerciseName::find()
        .into_model::<models::ExerciseName>()
        .all(&state.conn)
        .await?;

    Ok(Json(res))
}

pub async fn add_exercise_set_for_user(
    State(state): State<AppState>,
    Extension(user): Extension<user_login::Model>,
    Json(payload): Json<models::ExerciseSet>,
) -> Result<Json<()>> {
    // get or create exercise name
    let opt_name = ExerciseName::find()
        .filter(exercise_name::Column::Name.eq(payload.name()))
        .one(&state.conn)
        .await?;

    let name_id = if let Some(name) = opt_name {
        name.id
    } else {
        let new_name = exercise_name::ActiveModel {
            name: ActiveValue::Set(payload.name().to_string()),
            kind: ActiveValue::Set(payload.kind().into()),
            ..Default::default()
        };
        let res = ExerciseName::insert(new_name).exec(&state.conn).await?;
        res.last_insert_id
    };

    let new_exercise_set = exercise_set::ActiveModel {
        user_id: ActiveValue::Set(user.id),
        name_id: ActiveValue::Set(name_id),
        created_at: ActiveValue::Set(Utc::now().naive_utc()),
        ..payload.into()
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
        .column_as(exercise_name::Column::Kind, "kind")
        .order_by(exercise_set::Column::CreatedAt, Order::Desc)
        .join(
            JoinType::InnerJoin,
            exercise_set::Relation::ExerciseName.def(),
        );

    log::info!("{}", q.build(DbBackend::Sqlite).to_string());

    let res = q
        .into_model::<models::ExerciseSetJoinQuery>()
        .all(&state.conn)
        .await?;

    let res = res
        .into_iter()
        .map(|exsj| match exsj.kind {
            models::ExerciseKind::Weighted => {
                let exs: models::ExerciseSetWeightedQuery = exsj.try_into()?;
                Ok(models::ExerciseSetQuery::Weighted(exs))
            }
            models::ExerciseKind::Bodyweight => {
                let exs: models::ExerciseSetBodyweightQuery = exsj.try_into()?;
                Ok(models::ExerciseSetQuery::Bodyweight(exs))
            }
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(Json(res))
}

async fn get_weighted_exercise_sets_for_user(
    state: &AppState,
    user_id: i32,
) -> Result<Vec<models::ExerciseSetWeightedQuery>> {
    let q = ExerciseSet::find()
        .filter(exercise_set::Column::UserId.eq(user_id))
        .filter(exercise_name::Column::Kind.eq(models::ExerciseKind::Weighted))
        .column_as(exercise_name::Column::Name, "name")
        .join(
            JoinType::InnerJoin,
            exercise_set::Relation::ExerciseName.def(),
        );

    log::info!("{}", q.build(DbBackend::Sqlite).to_string());

    let res = q
        .into_model::<models::ExerciseSetWeightedQuery>()
        .all(&state.conn)
        .await?;

    Ok(res)
}

async fn get_bodyweight_exercise_sets_for_user(
    state: &AppState,
    user_id: i32,
) -> Result<Vec<models::ExerciseSetBodyweightQuery>> {
    let q = ExerciseSet::find()
        .filter(exercise_set::Column::UserId.eq(user_id))
        .filter(exercise_name::Column::Kind.eq(models::ExerciseKind::Bodyweight))
        .column_as(exercise_name::Column::Name, "name")
        .join(
            JoinType::InnerJoin,
            exercise_set::Relation::ExerciseName.def(),
        );

    let res = q
        .into_model::<models::ExerciseSetBodyweightQuery>()
        .all(&state.conn)
        .await?;

    Ok(res)
}
