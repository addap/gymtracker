use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub user_id: i32,
    pub name_id: i32,
    pub reps: i32,
    pub weight: f64,
    pub created_at: DateTime,
}
