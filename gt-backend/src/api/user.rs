use axum::extract::{Json, State};
use axum::Extension;
use chrono::Utc;
use email_address::EmailAddress;
use http::StatusCode;
use pbkdf2::password_hash::PasswordHash;
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use sea_orm::*;
use serde_json::{json, Value};

use crate::{AppError, AppState, Result};
use gt_core::entities::{prelude::*, *};
use gt_core::{models, models::UserAuth};

const DEBUG_TOKEN: &str = "secrettoken";

pub fn check_auth(token: &str) -> Option<i32> {
    if token.starts_with(DEBUG_TOKEN) {
        Some(
            token
                .chars()
                .nth(token.len() - 1)
                .unwrap()
                .to_digit(10)
                .unwrap() as i32,
        )
    } else {
        None
    }
}

/// Sign up new user and return an auth token on success.
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<models::UserSignup>,
) -> Result<Json<UserAuth>> {
    if !EmailAddress::is_valid(&payload.email)
        || payload.display_name.is_empty()
        || payload.username.is_empty()
        || payload.password.is_empty()
    {
        return Err(AppError::ValidationError);
    }

    // Hash password to PHC string ($pbkdf2-sha256$...)
    let salt = SaltString::generate(&mut OsRng);
    let pw_hash = Pbkdf2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| AppError::StatusCode(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .to_string();

    let new_user_login = user_login::ActiveModel {
        username: ActiveValue::Set(payload.username),
        email: ActiveValue::Set(payload.email),
        pw_hash: ActiveValue::Set(pw_hash),
        created_at: ActiveValue::Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    let new_user = UserLogin::insert(new_user_login).exec(&state.conn).await?;

    let new_user_info = user_info::ActiveModel {
        display_name: ActiveValue::Set(payload.display_name),
        user_id: ActiveValue::Set(new_user.last_insert_id),
        ..Default::default()
    };
    UserInfo::insert(new_user_info).exec(&state.conn).await?;

    let user_auth = UserAuth {
        auth_token: format!("{}-{}", DEBUG_TOKEN.to_string(), new_user.last_insert_id),
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
        .map_err(|_| AppError::Auth)?;

    let user_auth = UserAuth {
        auth_token: format!("{}-{}", DEBUG_TOKEN.to_string(), user_login.id),
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
