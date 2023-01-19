use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub name: String,
}
