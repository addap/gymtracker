use derive_more::From;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, From, PartialEq)]
pub struct MergeNames {
    pub to_delete: String,
    pub to_expand: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, From, PartialEq)]
pub struct AdminResetPassword {
    pub username: String,
    pub password: String,
}
