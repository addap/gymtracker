use axum::extract::{FromRequest, Json, State};
use http::{header::AUTHORIZATION, StatusCode};
use hyper::{Error, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use tower::{service_fn, Service, ServiceBuilder, ServiceExt};
use tower_http::auth::{AuthorizeRequest, RequireAuthorizationLayer};

use crate::AppState;

const DEBUG_TOKEN: &str = "secrettoken";

#[derive(Deserialize)]
pub struct UserLogin {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
struct UserAuth {
    auth_token: String,
}

/// Login with username + password(hash) and return an auth token on success.
pub async fn login(State(state): State<AppState>, Json(payload): Json<UserLogin>) -> Json<Value> {
    // check username + password
    // generate token and put into db
    let user_auth = UserAuth {
        auth_token: DEBUG_TOKEN.to_string(),
    };

    Json(json!(user_auth))
}

pub fn check_auth(token: &str) -> Option<i32> {
    if token == DEBUG_TOKEN {
        Some(1337)
    } else {
        None
    }
}
