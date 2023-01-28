#[cfg(not(target_arch = "wasm32"))]
pub mod db;
pub mod exercise;
pub mod user;

pub use exercise::*;
pub use user::*;
