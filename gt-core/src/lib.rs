#[cfg(not(target_arch = "wasm32"))]
pub mod entities;
pub mod models;

pub const APP_BASE: &str = "/app";
