use derive_more::{Deref, From};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserSignup {
    pub display_name: String,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Deref, From)]
pub struct AuthToken(pub String);

impl From<&str> for AuthToken {
    fn from(token: &str) -> Self {
        Self(token.to_string())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserAuth {
    pub username: String,
    pub id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInfo {
    pub display_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInfoTs {
    pub height: Option<f32>,
    pub weight: Option<f32>,
    pub muscle_mass: Option<f32>,
    pub body_fat: Option<f32>,
}
