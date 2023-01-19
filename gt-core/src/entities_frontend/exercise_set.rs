use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub reps: i32,
    pub weight: f32,
    pub created_at: DateTime,
}
