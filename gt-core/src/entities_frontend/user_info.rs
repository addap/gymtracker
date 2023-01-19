use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub user_id: i32,
    pub display_name: String,
    pub photo: Option<Vec<u8>>,
}
