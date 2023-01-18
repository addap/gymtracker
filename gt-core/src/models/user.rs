use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserSignup {
    pub display_name: String,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserAuth {
    pub auth_token: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserInfo {
    pub display_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserInfoTs {
    pub height: Option<f32>,
    pub weight: Option<f32>,
    pub muscle_mass: Option<f32>,
    pub body_fat: Option<f32>,
}
