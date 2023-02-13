use std::fmt::Display;

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

impl Display for AuthToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl From<&str> for AuthToken {
    fn from(token: &str) -> Self {
        Self(token.to_string())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserAuth {
    pub username: String,
    pub id: i32,
    pub is_superuser: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInfo {
    pub display_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInfoTs {
    pub height: Option<f64>,
    pub weight: Option<f64>,
    pub muscle_mass: Option<f64>,
    pub body_fat: Option<f64>,
}
