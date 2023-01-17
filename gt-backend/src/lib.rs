use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub mod api;
pub mod db;

#[derive(Clone)]
pub struct InnerAppState {
    pub conn: DatabaseConnection,
}

pub type AppState = Arc<InnerAppState>;
