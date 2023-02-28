use itertools::Itertools;
use ordered_float::OrderedFloat;
use sea_orm::*;
use std::collections::HashMap;

use crate::Result;
use gt_core::entities::{prelude::*, *};
use gt_core::models;

pub async fn get_weighted_exercise_set_prs_for_user(
    user_id: i32,
    conn: &DatabaseConnection,
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
        .all(conn)
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

        prs.push(models::PRWeightedQuery { name, pr });
    }

    Ok(prs)
}

pub async fn get_bodyweight_exercise_set_prs_for_user(
    user_id: i32,
    conn: &DatabaseConnection,
) -> Result<Vec<models::PRBodyweightQuery>> {
    let q = ExerciseSet::find()
        .filter(exercise_set::Column::UserId.eq(user_id))
        .column_as(exercise_name::Column::Name, "name")
        .filter(exercise_name::Column::Kind.eq(models::ExerciseKind::Bodyweight))
        .join(
            JoinType::InnerJoin,
            exercise_set::Relation::ExerciseName.def(),
        );

    log::info!("{}", q.build(DbBackend::Sqlite).to_string());

    let res = q
        .into_model::<models::ExerciseSetBodyweightQuery>()
        .all(conn)
        .await?;

    let mut data_per_exercise: HashMap<String, Vec<i32>> = HashMap::with_capacity(res.len());
    for exs in res {
        let prs = data_per_exercise.entry(exs.name).or_insert(Vec::new());
        prs.push(exs.reps);
    }

    let mut prs = Vec::with_capacity(data_per_exercise.len());
    for (name, mut data) in data_per_exercise.into_iter().sorted_by_key(|x| x.0.clone()) {
        data.sort_by(|a, b| b.cmp(a));
        let pr = data.into_iter().unique_by(|reps| *reps).take(3).collect();

        prs.push(models::PRBodyweightQuery { name, pr });
    }

    Ok(prs)
}
