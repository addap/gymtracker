use sea_orm::DatabaseConnection;

pub mod api;
pub mod db;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}
