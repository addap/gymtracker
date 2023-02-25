use axum::extract::Path;
use axum::{extract::State, Extension, Json};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use sea_orm::*;
use std::collections::HashMap;

use crate::{db, AppError, AppState, Result};
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
        if Ok(payload.kind()) != name.kind.try_into() {
            return Err(AppError::ValidationError);
        }
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
    let res = db::exercise::get_exercise_sets(user.id, None, &state.conn).await?;
    Ok(Json(res))
}

pub async fn get_paged_exercise_sets_for_user(
    State(state): State<AppState>,
    Extension(user): Extension<user_login::Model>,
    Path(page_size): Path<u64>,
) -> Result<Json<Vec<models::ExerciseSetQuery>>> {
    let res = db::exercise::get_exercise_sets(user.id, Some(page_size), &state.conn).await?;
    Ok(Json(res))
}

pub async fn get_exercise_set_prs_for_user(
    State(state): State<AppState>,
    Extension(user): Extension<user_login::Model>,
) -> Result<Json<models::PRQuery>> {
    let res_weighted = get_weighted_exercise_set_prs_for_user(&state, user.id).await?;

    let res = models::PRQuery {
        weighted: res_weighted,
    };

    Ok(Json(res))
}

pub async fn get_weighted_exercise_set_prs_for_user(
    state: &AppState,
    user_id: i32,
) -> Result<Vec<models::PRWeightedQuery>> {
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

    let mut prs = Vec::with_capacity(data_per_exercise.len());
    for (name, mut data) in data_per_exercise.into_iter().sorted_by_key(|x| x.0.clone()) {
        data.sort_by(|a, b| b.0.total_cmp(&a.0).then(b.1.cmp(&a.1)));
        let pr = data
            .into_iter()
            .unique_by(|(weight, reps)| (OrderedFloat(*weight), *reps))
            .take(3)
            .collect();

        prs.push(models::PRWeightedQuery {
            name: name.clone(),
            pr,
        });
    }

    Ok(prs)
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

pub async fn delete_exercise_set_for_user(
    State(state): State<AppState>,
    Extension(user): Extension<user_login::Model>,
    Json(payload): Json<models::ExerciseSetDelete>,
) -> Result<Json<()>> {
    let _res = ExerciseSet::delete_by_id(payload.id)
        .filter(exercise_set::Column::UserId.eq(user.id))
        .exec(&state.conn)
        .await?;

    Ok(Json(()))
}
