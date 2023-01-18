use axum::response::IntoResponse;
use http::StatusCode;
use migration::DbErr;
use sea_orm::DatabaseConnection;
use std::{result, sync::Arc};
use thiserror::Error;

pub mod api;
pub mod db;

#[derive(Clone)]
pub struct InnerAppState {
    pub conn: DatabaseConnection,
}

pub type AppState = Arc<InnerAppState>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error.")]
    Database(#[from] DbErr),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Database(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error.").into_response()
            }
        }
    }
}

pub type Result<T> = result::Result<T, AppError>;
