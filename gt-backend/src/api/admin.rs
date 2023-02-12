use axum::{extract::State, Extension, Json};
use migration::Expr;
use sea_orm::*;

use crate::{AppError, AppState, Result};
use gt_core::entities::{prelude::*, *};
use gt_core::models;

pub async fn merge_names(
    State(state): State<AppState>,
    #[allow(unused_variables)] Extension(user): Extension<user_login::Model>,
    Json(payload): Json<models::MergeNames>,
) -> Result<Json<u64>> {
    let delete_name = ExerciseName::find()
        .filter(exercise_name::Column::Name.eq(payload.to_delete.clone()))
        .one(&state.conn)
        .await?
        .ok_or(AppError::ResourceNotFound)?;
    let expand_name = ExerciseName::find()
        .filter(exercise_name::Column::Name.eq(payload.to_expand.clone()))
        .one(&state.conn)
        .await?
        .ok_or(AppError::ResourceNotFound)?;

    let q = ExerciseSet::update_many()
        .col_expr(exercise_set::Column::NameId, Expr::value(expand_name.id))
        .filter(exercise_set::Column::NameId.eq(delete_name.id));

    log::info!("{}", q.build(DbBackend::Sqlite).to_string());

    let res_update = q.exec(&state.conn).await?;
    let _res_delete = delete_name.delete(&state.conn).await?;

    Ok(Json(res_update.rows_affected))
}
