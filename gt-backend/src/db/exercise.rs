use chrono::NaiveDate;
use itertools::Itertools;
use sea_orm::*;
use std::collections::HashMap;

use crate::Result;
use gt_core::entities::{prelude::*, *};
use gt_core::models;

pub async fn get_exercise_sets(
    user_id: i32,
    limit_opt: Option<u64>,
    conn: &DatabaseConnection,
) -> Result<Vec<models::ExerciseSetQuery>> {
    let mut q = ExerciseSet::find()
        .filter(exercise_set::Column::UserId.eq(user_id))
        .column_as(exercise_name::Column::Name, "name")
        .column_as(exercise_name::Column::Kind, "kind")
        .order_by(exercise_set::Column::CreatedAt, Order::Desc)
        .join(
            JoinType::InnerJoin,
            exercise_set::Relation::ExerciseName.def(),
        );

    if let Some(limit) = limit_opt {
        q = q.limit(limit)
    }

    log::info!("{}", q.build(DbBackend::Postgres).to_string());

    let res = q
        .into_model::<models::ExerciseSetJoinQuery>()
        .all(conn)
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

    Ok(res)
}

pub async fn get_exercise_graphs(
    user_id: i32,
    conn: &DatabaseConnection,
) -> Result<Vec<models::ExerciseGraphQuery>> {
    let q = ExerciseSet::find()
        .filter(exercise_set::Column::UserId.eq(user_id))
        .filter(exercise_name::Column::Kind.eq(models::ExerciseKind::Weighted))
        .column_as(exercise_name::Column::Name, "name")
        .order_by(exercise_set::Column::NameId, Order::Asc)
        .join(
            JoinType::InnerJoin,
            exercise_set::Relation::ExerciseName.def(),
        );

    log::info!("{}", q.build(DbBackend::Postgres).to_string());

    let data = q
        .into_model::<models::ExerciseGraphJoinQuery>()
        .all(conn)
        .await?;

    let mut data_per_name: HashMap<String, HashMap<NaiveDate, Vec<(f64, i32)>>> = HashMap::new();

    for jq in data {
        // If we have not added data for this exercise, insert a new HashMap for this exercise.
        if !data_per_name.contains_key(&jq.name) {
            data_per_name.insert(jq.name.clone(), HashMap::new());
        }
        let data_per_date = data_per_name.get_mut(&jq.name).unwrap();

        // If we have not added data for this exercise for this date, insert a new Vector for this date.
        if !data_per_date.contains_key(&jq.created_at.date()) {
            data_per_date.insert(jq.created_at.date(), Vec::new());
        }
        let data_weights = data_per_date.get_mut(&jq.created_at.date()).unwrap();

        // If we have added both before, extend the existing Vector.
        data_weights.push((jq.weight, jq.reps));
    }

    let res = data_per_name
        .into_iter()
        .sorted_by(|(name1, _), (name2, _)| name1.cmp(&name2))
        .map(|(name, per_date_map)| {
            let per_date = per_date_map
                .into_iter()
                .sorted_by(|(date1, _), (date2, _)| date1.cmp(&date2))
                .map(|(date, weights)| models::ExerciseGraphQueryPerDate { date, weights })
                .collect();
            models::ExerciseGraphQuery {
                name: name.to_string(),
                per_date,
            }
        })
        .collect();
    Ok(res)
}

// pub async fn get_weighted_exercise_sets_for_user(
//     state: &AppState,
//     user_id: i32,
// ) -> Result<Vec<models::ExerciseSetWeightedQuery>> {
//     let q = ExerciseSet::find()
//         .filter(exercise_set::Column::UserId.eq(user_id))
//         .filter(exercise_name::Column::Kind.eq(models::ExerciseKind::Weighted))
//         .column_as(exercise_name::Column::Name, "name")
//         .order_by(exercise_set::Column::CreatedAt, Order::Desc)
//         .join(
//             JoinType::InnerJoin,
//             exercise_set::Relation::ExerciseName.def(),
//         );

//     log::info!("{}", q.build(DbBackend::Postgres).to_string());

//     let res = q
//         .into_model::<models::ExerciseSetWeightedQuery>()
//         .all(&state.conn)
//         .await?;

//     Ok(res)
// }

// pub async fn get_bodyweight_exercise_sets_for_user(
//     state: &AppState,
//     user_id: i32,
// ) -> Result<Vec<models::ExerciseSetBodyweightQuery>> {
//     let q = ExerciseSet::find()
//         .filter(exercise_set::Column::UserId.eq(user_id))
//         .filter(exercise_name::Column::Kind.eq(models::ExerciseKind::Bodyweight))
//         .column_as(exercise_name::Column::Name, "name")
//         .order_by(exercise_set::Column::CreatedAt, Order::Desc)
//         .join(
//             JoinType::InnerJoin,
//             exercise_set::Relation::ExerciseName.def(),
//         );

//     let res = q
//         .into_model::<models::ExerciseSetBodyweightQuery>()
//         .all(&state.conn)
//         .await?;

//     Ok(res)
// }
