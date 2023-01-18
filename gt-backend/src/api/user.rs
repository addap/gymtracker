use axum::extract::{Json, State};
use axum::Extension;
use chrono::Utc;
use sea_orm::*;
use serde_json::{json, Value};

use crate::{AppError, AppState, Result};
use gt_core::entities::{prelude::*, *};
use gt_core::{models, models::UserAuth};

const DEBUG_TOKEN: &str = "secrettoken";

pub fn check_auth(token: &str) -> Option<i32> {
    if token == DEBUG_TOKEN {
        Some(1337)
    } else {
        None
    }
}

/// Sign up new user and return an auth token on success.
pub async fn register(
    State(state): State<AppState>,
    Json(paylod): Json<models::UserSignup>,
) -> Result<Json<UserAuth>> {
    // put user in db, then geneerate an auth token
    let user_auth = UserAuth {
        auth_token: DEBUG_TOKEN.to_string(),
    };

    Ok(Json(user_auth))
}

/// Login with username + password(hash) and return an auth token on success.
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<models::UserLogin>,
) -> Result<Json<UserAuth>> {
    // check username + password
    // generate token and put into db
    let user_auth = UserAuth {
        auth_token: DEBUG_TOKEN.to_string(),
    };

    Ok(Json(user_auth))
}

pub async fn logout(
    State(state): State<AppState>,
    Extension(user): Extension<user_login::Model>,
) -> Result<Json<Value>> {
    Ok(Json(json!(())))
}

pub async fn change_user_info(
    State(state): State<AppState>,
    Extension(user): Extension<user_login::Model>,
    Json(payload): Json<models::UserInfo>,
) -> Result<Json<()>> {
    let old_user_info = user
        .find_related(UserInfo)
        .one(&state.conn)
        .await?
        .ok_or(AppError::ResourceNotFound)?;

    let new_user_info = user_info::ActiveModel {
        id: ActiveValue::Set(old_user_info.id),
        display_name: ActiveValue::Set(payload.display_name),
        ..Default::default()
    };
    new_user_info.update(&state.conn).await?;

    Ok(Json(()))
}

pub async fn get_user_info(
    State(state): State<AppState>,
    Extension(user): Extension<user_login::Model>,
) -> Result<Json<user_info::Model>> {
    let user_info = user
        .find_related(UserInfo)
        .one(&state.conn)
        .await?
        .ok_or(AppError::ResourceNotFound)?;

    Ok(Json(user_info))
}

pub async fn add_user_info_ts(
    State(state): State<AppState>,
    Extension(user): Extension<user_login::Model>,
    Json(payload): Json<models::UserInfoTs>,
) -> Result<Json<()>> {
    let new_user_info_ts = user_info_ts::ActiveModel {
        user_id: ActiveValue::Set(user.id),
        height: ActiveValue::Set(payload.height),
        weight: ActiveValue::Set(payload.weight),
        muscle_mass: ActiveValue::Set(payload.muscle_mass),
        body_fat: ActiveValue::Set(payload.body_fat),
        created_at: ActiveValue::Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    UserInfoTs::insert(new_user_info_ts)
        .exec(&state.conn)
        .await?;

    Ok(Json(()))
}

pub async fn get_user_info_ts(
    State(state): State<AppState>,
    Extension(user): Extension<user_login::Model>,
) -> Result<Json<Vec<user_info_ts::Model>>> {
    let res = user.find_related(UserInfoTs).all(&state.conn).await?;

    Ok(Json(res))
}
