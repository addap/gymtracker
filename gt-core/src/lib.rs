#[cfg(not(target_arch = "wasm32"))]
mod entities_backend;
#[cfg(target_arch = "wasm32")]
mod entities_frontend;

pub mod models;
pub mod entities {
    #[cfg(not(target_arch = "wasm32"))]
    pub use super::entities_backend::*;
    #[cfg(target_arch = "wasm32")]
    pub use super::entities_frontend::*;
}
