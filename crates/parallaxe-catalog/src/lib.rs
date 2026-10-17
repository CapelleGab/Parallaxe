//! SQLite catalog of Parallaxe. The only crate allowed to touch the database file.

mod catalog;
mod error;
mod migrations;

pub use catalog::Catalog;
pub use error::CatalogError;
pub use migrations::SCHEMA_VERSION;
