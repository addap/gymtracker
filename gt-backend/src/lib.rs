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
    #[error("{0}")]
    Database(#[from] DbErr),
    #[error("Authentication error.")]
    Auth,
    #[error("Resource not found.")]
    ResourceNotFound,
    #[error("{1}")]
    StatusCode(StatusCode, String),
    #[error("Form validation failed.")]
    ValidationError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let msg = self.to_string();
        let status_code = match self {
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Auth => StatusCode::UNAUTHORIZED,
            AppError::ResourceNotFound => StatusCode::NOT_FOUND,
            AppError::StatusCode(status_code, _) => status_code,
            AppError::ValidationError => StatusCode::BAD_REQUEST,
        };

        (status_code, msg).into_response()
    }
}

pub type Result<T> = result::Result<T, AppError>;
