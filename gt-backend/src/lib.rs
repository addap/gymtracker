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
    pub secret: String,
}

pub type AppState = Arc<InnerAppState>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("GT: {0}")]
    Database(#[from] DbErr),
    #[error("GT: Authentication error.")]
    Auth,
    #[error("GT: Resource not found.")]
    ResourceNotFound,
    #[error("GT: {1}")]
    StatusCode(StatusCode, String),
    #[error("GT: Form validation failed.")]
    ValidationError,
    #[error("GT: {0}")]
    Generic(#[from] anyhow::Error),
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
            AppError::Generic(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, msg).into_response()
    }
}

pub type Result<T> = result::Result<T, AppError>;
