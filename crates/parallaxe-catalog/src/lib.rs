//! SQLite catalog of Parallaxe. The only crate allowed to touch the database file.

mod catalog;
mod error;
mod libraries;
mod migrations;
mod rows;

#[cfg(test)]
mod schema_tests;

pub use catalog::Catalog;
pub use error::CatalogError;
pub use migrations::SCHEMA_VERSION;
