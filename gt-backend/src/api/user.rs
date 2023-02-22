use axum::{
    body::StreamBody,
    extract::{Json, State},
    http::header,
    response::IntoResponse,
    Extension,
};
use chrono::Utc;
use gt_core::models::UserAuth;
use http::{HeaderMap, StatusCode};
use pbkdf2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Pbkdf2,
};
use sea_orm::*;
use tokio::io::AsyncReadExt;
use tokio_util::io::ReaderStream;

use crate::{db, AppError, AppState, Result};
use gt_core::auth::create_token;
use gt_core::entities::{prelude::*, *};
use gt_core::{models, models::AuthToken};

/// Sign up new user and return an auth token on success.
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<models::UserSignup>,
) -> Result<Json<AuthToken>> {
    let last_insert_id = db::user::create_user(&payload, false, &state.conn).await?;

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
) -> Result<Json<models::UserInfoQuery>> {
    let res = db::user::get_user_info(user, &state.conn).await?;
    Ok(Json(res))
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

pub async fn get_user_picture(
    State(state): State<AppState>,
    Extension(user): Extension<user_login::Model>,
) -> Result<impl IntoResponse> {
    let user_info = user
        .find_related(UserInfo)
        .one(&state.conn)
        .await?
        .ok_or(AppError::ResourceNotFound)?;

    if let Some(bytes) = user_info.photo {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            "text/toml; charset=utf-8".parse().unwrap(),
        );
        // headers.insertheader::CONTENT_DISPOSITION, "attachment; filename=\"Cargo.toml\"",

        Ok((headers, bytes))
    } else {
        let mut file = match tokio::fs::File::open("gt-backend/static/default_picture.jpg").await {
            Ok(file) => file,
            Err(_) => return Err(AppError::ResourceNotFound),
        };
        let mut content = vec![];
        file.read_to_end(&mut content).await.unwrap();
        // // convert the `AsyncRead` into a `Stream`
        // let stream = ReaderStream::new(file);
        // // convert the `Stream` into an `axum::body::HttpBody`
        // let body = StreamBody::new(stream);

        let mut headers = HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            "text/toml; charset=utf-8".parse().unwrap(),
        );
        // headers.insertheader::CONTENT_DISPOSITION, "attachment; filename=\"Cargo.toml\"",

        Ok((headers, content))
    }
}

pub async fn change_user_picture(
    State(state): State<AppState>,
    Extension(user): Extension<user_login::Model>,
) -> Result<Json<()>> {
    Ok(Json(()))
}
