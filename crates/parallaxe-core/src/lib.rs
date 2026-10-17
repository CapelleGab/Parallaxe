//! Business rules of Parallaxe, independent of Tauri and of any UI.

pub mod asset;
pub mod error;
pub mod id;
pub mod library;
pub mod metadata;
pub mod name;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
