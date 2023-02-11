use anyhow::anyhow;
use axum::extract::{State, TypedHeader};
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::middleware::Next;
use axum::response::Response;
use axum::Extension;
use axum::Json;
use chrono::Utc;
use hmac::{Hmac, Mac};
use hyper::Request;
use jwt::{SignWithKey, VerifyWithKey};
use sea_orm::EntityTrait;
use sha2::Sha256;
use std::collections::HashMap;

use crate::{AppError, AppState, Result};
use gt_core::entities::{prelude::*, *};
use gt_core::models::{AuthToken, UserAuth};

pub fn verify_token(state: &AppState, token: AuthToken) -> Result<i32> {
    let key: Hmac<Sha256> =
        Hmac::new_from_slice(state.secret.as_bytes()).map_err(|e| anyhow!(e))?;

    let claims: HashMap<String, String> = token.verify_with_key(&key).map_err(|e| anyhow!(e))?;

    let user_id = claims.get("sub").ok_or(AppError::Auth)?;
    let user_id: i32 = user_id.parse().map_err(|_| AppError::Auth)?;

    Ok(user_id)
}

pub fn create_token(state: &AppState, user: UserAuth) -> Result<AuthToken> {
    let key: Hmac<Sha256> =
        Hmac::new_from_slice(state.secret.as_bytes()).map_err(|e| anyhow!(e))?;

    let mut claims = HashMap::new();
    claims.insert("sub", user.id.to_string());
    claims.insert("name", user.username);
    claims.insert("iat", Utc::now().naive_utc().to_string());

    let token = claims.sign_with_key(&key).map_err(|e| anyhow!(e))?;

    Ok(token.into())
}

pub async fn auth_middleware<B>(
    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let user_id = verify_token(&state, auth_header.token().into())?;
    let user = UserLogin::find_by_id(user_id)
        .one(&state.conn)
        .await?
        .ok_or(AppError::ResourceNotFound)?;

    // Set `user` as a request extension so it can be accessed by other
    // services down the stack.
    request.extensions_mut().insert(user);
    let response = next.run(request).await;

    Ok(response)
}

pub async fn superuser_middleware<B>(
    Extension(user): Extension<user_login::Model>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    if !user.is_superuser {
        return Err(AppError::Auth);
    }

    let response = next.run(request).await;

    Ok(response)
}

/// Empty endpoint which should be behind the auth middleware so that the client can check the validity of its token.
pub async fn check_token() -> Result<Json<()>> {
    Ok(Json(()))
}
