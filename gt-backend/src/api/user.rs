use axum::extract::{Json, State};
use axum::Extension;
use chrono::Utc;
use gt_core::models::UserAuth;
use http::StatusCode;
use pbkdf2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Pbkdf2,
};
use sea_orm::*;

use crate::{db, AppError, AppState, Result};
use gt_core::auth::create_token;
use gt_core::entities::{prelude::*, *};
use gt_core::{models, models::AuthToken};

/// Sign up new user and return an auth token on success.
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<models::UserSignup>,
) -> Result<Json<AuthToken>> {
    let last_insert_id = db::create_user(&payload, false, &state).await?;

    let auth_token = create_token(
        &state.secret,
        UserAuth {
            username: payload.username,
            id: last_insert_id,
            is_superuser: false,
        },
    )?;

    Ok(Json(auth_token))
}

/// Login with username + password(hash) and return an auth token on success.
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<models::UserLogin>,
) -> Result<Json<AuthToken>> {
    // check username + password
    // generate token and put into db
    let user_login = UserLogin::find()
        .filter(user_login::Column::Username.eq(&payload.username[..]))
        .one(&state.conn)
        .await?
        .ok_or(AppError::ResourceNotFound)?;

    let pw_hash = PasswordHash::new(&user_login.pw_hash).map_err(|_| {
        AppError::StatusCode(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Malformed Hash."),
        )
    })?;

    Pbkdf2
        .verify_password(payload.password.as_bytes(), &pw_hash)
        .map_err(|_| AppError::ValidationError)?;

    let auth_token = create_token(
        &state.secret,
        UserAuth {
            username: user_login.username,
            id: user_login.id,
            is_superuser: user_login.is_superuser,
        },
    )?;

    Ok(Json(auth_token))
}

#[allow(unused_variables)]
pub async fn logout(
    State(state): State<AppState>,
    Extension(user): Extension<user_login::Model>,
) -> Result<Json<()>> {
    Ok(Json(()))
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
