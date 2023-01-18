use axum::extract::{State, TypedHeader};
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::middleware::Next;
use axum::response::Response;
use hyper::Request;
use sea_orm::EntityTrait;

use crate::{AppError, AppState};
use gt_core::entities::prelude::*;

pub mod exercise;
pub mod user;

pub async fn auth_middleware<B>(
    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    if let Some(user_id) = user::check_auth(auth_header.token()) {
        let user = UserLogin::find_by_id(user_id)
            .one(&state.conn)
            .await?
            .ok_or(AppError::ResourceNotFound)?;

        // Set `user` as a request extension so it can be accessed by other
        // services down the stack.
        request.extensions_mut().insert(user);
        let response = next.run(request).await;

        Ok(response)
    } else {
        Err(AppError::Auth)
    }
}
