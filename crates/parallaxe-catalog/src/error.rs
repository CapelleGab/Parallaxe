#[derive(Debug, thiserror::Error)]
pub enum CatalogError {
    #[error("database error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("catalog schema v{found} is newer than supported v{supported}")]
    SchemaTooNew { found: usize, supported: usize },
}
