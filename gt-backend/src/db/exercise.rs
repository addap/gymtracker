use sea_orm::*;

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
