use axum::extract::{State, TypedHeader};
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::middleware::Next;
use axum::response::Response;
use axum::Extension;
use axum::Json;
use hyper::Request;
use sea_orm::EntityTrait;

use crate::{AppError, AppState, Result};
use gt_core::auth::verify_token;
use gt_core::entities::{prelude::*, *};

pub async fn jwt_middleware<B>(
    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let user_id = verify_token(&state.secret, &auth_header.token().into())?;
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

/// Empty endpoint which should be behind the jwt middleware so that the client can check the validity of its token.
pub async fn check_token() -> Result<Json<()>> {
    Ok(Json(()))
}
