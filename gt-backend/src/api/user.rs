use axum::extract::{Json, State};
use serde_json::{json, Value};

use crate::{AppState, Result};
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
) -> Result<Json<Value>> {
    // put user in db, then geneerate an auth token
    let user_auth = UserAuth {
        auth_token: DEBUG_TOKEN.to_string(),
    };

    Ok(Json(json!(user_auth)))
}

/// Login with username + password(hash) and return an auth token on success.
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<models::UserLogin>,
) -> Result<Json<Value>> {
    // check username + password
    // generate token and put into db
    let user_auth = UserAuth {
        auth_token: DEBUG_TOKEN.to_string(),
    };

    Ok(Json(json!(user_auth)))
}

pub async fn logout(State(state): State<AppState>) -> Result<Json<Value>> {
    Ok(Json(json!(())))
}

pub async fn change_user_info(
    State(state): State<AppState>,
    Json(payload): Json<models::UserInfo>,
) -> Result<Json<Value>> {
    //

    Ok(Json(json!(())))
}

pub async fn get_user_info(State(state): State<AppState>) -> Result<Json<user_info::Model>> {
    //
    Ok(Json(user_info::Model {
        id: 0,
        user_id: 0,
        display_name: String::from("Testuser"),
        photo: None,
    }))
}

pub async fn add_user_info_ts(
    State(state): State<AppState>,
    Json(payload): Json<models::UserInfoTs>,
) -> Result<Json<Value>> {
    //
    Ok(Json(json!(())))
}

pub async fn get_user_info_ts(
    State(state): State<AppState>,
) -> Result<Json<Vec<user_info_ts::Model>>> {
    //
    Ok(Json(vec![]))
}
