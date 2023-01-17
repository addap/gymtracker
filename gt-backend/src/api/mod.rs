use axum::body::Body;
use axum::extract::{FromRequest, Json, State};
use axum::middleware::Next;
use axum::response::Response;
use http::HeaderValue;
use http::{header::AUTHORIZATION, StatusCode};
use hyper::{Error, Request};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use tower::{service_fn, Service, ServiceBuilder, ServiceExt};
use tower_http::auth::{AuthorizeRequest, RequireAuthorizationLayer};

use crate::AppState;

pub mod exercise;
pub mod user;

pub async fn auth_middleware<B>(
    State(state): State<AppState>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, (StatusCode, &'static str)> {
    if let Some(auth) = request.headers().get(AUTHORIZATION) {
        let auth = auth.to_str().unwrap();

        if let Some(user_id) = user::check_auth(auth) {
            // Set `user_id` as a request extension so it can be accessed by other
            // services down the stack.
            request.extensions_mut().insert(user_id);
            let response = next.run(request).await;

            Ok(response)
        } else {
            Err((StatusCode::UNAUTHORIZED, "asd"))
        }
    } else {
        Err((StatusCode::UNAUTHORIZED, "def"))
    }
}
