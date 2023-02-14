use axum::extract::Path;
use axum::{extract::State, Extension, Json};
use chrono::Utc;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use sea_orm::*;
use std::collections::HashMap;

use crate::{db, AppState, Result};
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

pub async fn get_all_exercise_sets_for_user(
    State(state): State<AppState>,
    Extension(user): Extension<user_login::Model>,
) -> Result<Json<Vec<models::ExerciseSetQuery>>> {
    let res = db::get_exercise_sets(user.id, None, &state.conn).await?;
    Ok(Json(res))
}

pub async fn get_paged_exercise_sets_for_user(
    State(state): State<AppState>,
    Extension(user): Extension<user_login::Model>,
    Path(page_size): Path<u64>,
) -> Result<Json<Vec<models::ExerciseSetQuery>>> {
    let res = db::get_exercise_sets(user.id, Some(page_size), &state.conn).await?;
    Ok(Json(res))
}

pub async fn get_exercise_set_prs_for_user(
    State(state): State<AppState>,
    Extension(user): Extension<user_login::Model>,
) -> Result<Json<models::PRQuery>> {
    let (res_weighted_weight, res_weighted_reps) =
        get_weighted_exercise_set_prs_for_user(&state, user.id).await?;

    let res = models::PRQuery {
        weighted_weight: res_weighted_weight,
        weighted_reps: res_weighted_reps,
    };

    Ok(Json(res))
}

pub async fn get_weighted_exercise_set_prs_for_user(
    state: &AppState,
    user_id: i32,
) -> Result<(
    Vec<models::PRWeightedQueryWeight>,
    Vec<models::PRWeightedQueryReps>,
)> {
    let q = ExerciseSet::find()
        .filter(exercise_set::Column::UserId.eq(user_id))
        .column_as(exercise_name::Column::Name, "name")
        .filter(exercise_name::Column::Kind.eq(models::ExerciseKind::Weighted))
        .join(
            JoinType::InnerJoin,
            exercise_set::Relation::ExerciseName.def(),
        );

    log::info!("{}", q.build(DbBackend::Sqlite).to_string());

    let res = q
        .into_model::<models::ExerciseSetWeightedQuery>()
        .all(&state.conn)
        .await?;

    let mut data_per_exercise: HashMap<String, Vec<(f64, i32)>> = HashMap::with_capacity(res.len());
    for exs in res {
        let prs = data_per_exercise.entry(exs.name).or_insert(Vec::new());
        prs.push((exs.weight, exs.reps));
    }

    let mut prs_weight = Vec::with_capacity(data_per_exercise.len());
    let mut prs_reps = Vec::with_capacity(data_per_exercise.len());
    for (name, mut data) in data_per_exercise.into_iter().sorted_by_key(|x| x.0.clone()) {
        data.sort_by(|a, b| b.0.total_cmp(&a.0));
        let pr_weight = data
            .iter()
            .map(|(weight, _)| *weight)
            .unique_by(|f| OrderedFloat(*f))
            .take(3)
            .collect();

        data.sort_by(|a, b| b.1.cmp(&a.1));
        let pr_reps = data
            .iter()
            .map(|(_, reps)| *reps)
            .unique()
            .take(3)
            .collect();

        prs_weight.push(models::PRWeightedQueryWeight {
            name: name.clone(),
            pr_weight,
        });
        prs_reps.push(models::PRWeightedQueryReps { name, pr_reps });
    }

    Ok((prs_weight, prs_reps))
}

pub async fn get_weighted_exercise_sets_for_user(
    state: &AppState,
    user_id: i32,
) -> Result<Vec<models::ExerciseSetWeightedQuery>> {
    let q = ExerciseSet::find()
        .filter(exercise_set::Column::UserId.eq(user_id))
        .filter(exercise_name::Column::Kind.eq(models::ExerciseKind::Weighted))
        .column_as(exercise_name::Column::Name, "name")
        .order_by(exercise_set::Column::CreatedAt, Order::Desc)
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

pub async fn get_bodyweight_exercise_sets_for_user(
    state: &AppState,
    user_id: i32,
) -> Result<Vec<models::ExerciseSetBodyweightQuery>> {
    let q = ExerciseSet::find()
        .filter(exercise_set::Column::UserId.eq(user_id))
        .filter(exercise_name::Column::Kind.eq(models::ExerciseKind::Bodyweight))
        .column_as(exercise_name::Column::Name, "name")
        .order_by(exercise_set::Column::CreatedAt, Order::Desc)
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
