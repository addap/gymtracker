use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub user_id: i32,
    pub height: Option<f64>,
    pub weight: Option<f64>,
    pub muscle_mass: Option<f64>,
    pub body_fat: Option<f64>,
    pub created_at: DateTime,
}
