use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub pw_hash: String,
    pub created_at: DateTime,
}
