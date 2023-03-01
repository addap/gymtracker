use axum::extract::Path;
use axum::{extract::State, Extension, Json};
use migration::{Expr, Query, SimpleExpr, SubQueryStatement};
use sea_orm::*;

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
    Extension(user): Extension<user_login::Model>,
) -> Result<Json<Vec<models::ExerciseNameQuery>>> {
    let res = ExerciseName::find()
        .column_as(
            SimpleExpr::SubQuery(
                None,
                Box::new(SubQueryStatement::SelectStatement(
                    Query::select()
                        .column((exercise_set::Entity, exercise_set::Column::Weight))
                        .from(exercise_set::Entity)
                        .and_where(exercise_set::Column::UserId.eq(user.id))
                        .and_where(
                            Expr::col(exercise_set::Column::NameId)
                                .equals(exercise_name::Entity, exercise_name::Column::Id),
                        )
                        .order_by(exercise_set::Column::CreatedAt, Order::Desc)
                        .limit(1)
                        .to_owned(),
                )),
            ),
            "last_weight",
        )
        .into_model::<models::ExerciseNameQuery>()
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
    let res_weighted = db::pr::get_weighted_exercise_set_prs_for_user(user.id, &state.conn).await?;
    let res_bodyweight =
        db::pr::get_bodyweight_exercise_set_prs_for_user(user.id, &state.conn).await?;

    let res = models::PRQuery {
        weighted: res_weighted,
        bodyweight: res_bodyweight,
    };

    Ok(Json(res))
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
