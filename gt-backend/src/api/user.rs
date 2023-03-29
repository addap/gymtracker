use axum::{
    body::Bytes,
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
    let mut user_info: user_info::ActiveModel = user
        .find_related(UserInfo)
        .one(&state.conn)
        .await?
        .ok_or(AppError::ResourceNotFound)?
        .into();

    user_info.display_name = ActiveValue::Set(payload.display_name);
    user_info.update(&state.conn).await?;

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
            "application/octet-stream".parse().unwrap(),
        );

        Ok((headers, bytes))
    } else {
        // // convert the `AsyncRead` into a `Stream`
        // let stream = ReaderStream::new(file);
        // // convert the `Stream` into an `axum::body::HttpBody`
        // let body = StreamBody::new(stream);

        // let mut headers = HeaderMap::new();
        // headers.insert(
        //     header::CONTENT_TYPE,
        //     "application/octet-stream".parse().unwrap(),
        // );

        // // TODO can we avoid cloning here?
        // Ok((headers, DEFAULT_PIC.clone()))
        Err(AppError::ResourceNotFound)
    }
}

pub async fn change_user_picture(
    State(state): State<AppState>,
    Extension(user): Extension<user_login::Model>,
    bytes: Bytes,
) -> Result<Json<()>> {
    let mut user_info: user_info::ActiveModel = user
        .find_related(UserInfo)
        .one(&state.conn)
        .await?
        .ok_or(AppError::ResourceNotFound)?
        .into();

    user_info.photo = ActiveValue::Set(Some(bytes.to_vec()));
    user_info.update(&state.conn).await?;

    Ok(Json(()))
}

pub async fn delete_user_picture(
    State(state): State<AppState>,
    Extension(user): Extension<user_login::Model>,
) -> Result<Json<()>> {
    let mut user_info: user_info::ActiveModel = user
        .find_related(UserInfo)
        .one(&state.conn)
        .await?
        .ok_or(AppError::ResourceNotFound)?
        .into();

    user_info.photo = ActiveValue::Set(None);
    user_info.update(&state.conn).await?;

    Ok(Json(()))
}
