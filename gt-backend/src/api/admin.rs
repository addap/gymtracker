use axum::{extract::State, Extension, Json};
use migration::Expr;
use sea_orm::*;

use crate::{db, AppError, AppState, Result};
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
    let expand_name_opt = ExerciseName::find()
        .filter(exercise_name::Column::Name.eq(payload.to_expand.clone()))
        .one(&state.conn)
        .await?;

    let expand_name_id = if let Some(expand_name) = expand_name_opt {
        if delete_name.kind != expand_name.kind {
            return Err(AppError::ValidationError);
        }
        expand_name.id
    } else {
        let new_name = exercise_name::ActiveModel {
            name: ActiveValue::Set(payload.to_expand.clone()),
            kind: ActiveValue::Set(delete_name.kind.into()),
            ..Default::default()
        };
        let res = ExerciseName::insert(new_name).exec(&state.conn).await?;
        res.last_insert_id
    };

    let q = ExerciseSet::update_many()
        .col_expr(exercise_set::Column::NameId, Expr::value(expand_name_id))
        .filter(exercise_set::Column::NameId.eq(delete_name.id));

    log::info!("{}", q.build(DbBackend::Postgres).to_string());

    let res_update = q.exec(&state.conn).await?;
    let _res_delete = delete_name.delete(&state.conn).await?;

    Ok(Json(res_update.rows_affected))
}

pub async fn reset_password(
    State(state): State<AppState>,
    #[allow(unused_variables)] Extension(user): Extension<user_login::Model>,
    Json(payload): Json<models::AdminResetPassword>,
) -> Result<Json<()>> {
    let new_pw_hash = db::user::hash_password(&payload.password)?;

    let mut user_model: user_login::ActiveModel = UserLogin::find()
        .filter(user_login::Column::Username.eq(payload.username))
        .one(&state.conn)
        .await?
        .ok_or(AppError::ResourceNotFound)?
        .into();
    user_model.pw_hash = ActiveValue::Set(new_pw_hash);
    user_model.update(&state.conn).await?;

    Ok(Json(()))
}
