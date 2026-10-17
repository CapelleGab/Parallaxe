use std::path::Path;
use std::time::Duration;

use rusqlite::Connection;

use crate::CatalogError;
use crate::migrations;

pub struct Catalog {
    conn: Connection,
}

impl Catalog {
    /// Opens or creates the catalog. Must live on a local disk: WAL is unsafe on network volumes.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, CatalogError> {
        let conn = Connection::open(path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        Self::init(conn)
    }

    pub fn open_in_memory() -> Result<Self, CatalogError> {
        Self::init(Connection::open_in_memory()?)
    }

    pub fn schema_version(&self) -> Result<usize, CatalogError> {
        migrations::schema_version(&self.conn)
    }

    fn init(mut conn: Connection) -> Result<Self, CatalogError> {
        conn.pragma_update(None, "foreign_keys", true)?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        conn.busy_timeout(Duration::from_secs(5))?;
        migrations::migrate(&mut conn)?;
        Ok(Self { conn })
    }

    #[cfg(test)]
    pub(crate) fn conn(&self) -> &Connection {
        &self.conn
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SCHEMA_VERSION;

    #[test]
    fn opens_in_memory_at_latest_version() {
        let catalog = Catalog::open_in_memory().unwrap();
        assert_eq!(catalog.schema_version().unwrap(), SCHEMA_VERSION);
    }

    #[test]
    fn enables_foreign_keys() {
        let catalog = Catalog::open_in_memory().unwrap();
        let enabled: bool = catalog
            .conn()
            .pragma_query_value(None, "foreign_keys", |row| row.get(0))
            .unwrap();
        assert!(enabled);
    }

    #[test]
    fn creates_a_wal_database_on_disk_and_reopens_it() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("catalog.db");

        let catalog = Catalog::open(&path).unwrap();
        let mode: String = catalog
            .conn()
            .pragma_query_value(None, "journal_mode", |row| row.get(0))
            .unwrap();
        assert_eq!(mode, "wal");
        drop(catalog);

        let reopened = Catalog::open(&path).unwrap();
        assert_eq!(reopened.schema_version().unwrap(), SCHEMA_VERSION);
    }
}
