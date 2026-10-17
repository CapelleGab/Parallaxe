use parallaxe_core::error::{CoreError, ErrorCode};
use parallaxe_core::name::NameError;
use rusqlite::ffi;

#[derive(Debug, thiserror::Error)]
pub enum CatalogError {
    #[error("database error: {0}")]
    Database(rusqlite::Error),

    #[error("catalog schema v{found} is newer than supported v{supported}")]
    SchemaTooNew { found: usize, supported: usize },

    #[error("{0} not found")]
    NotFound(&'static str),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("invalid argument: {0}")]
    InvalidArgument(String),
}

// `?` on any rusqlite call goes through here, so constraint violations get a meaningful kind.
impl From<rusqlite::Error> for CatalogError {
    fn from(error: rusqlite::Error) -> Self {
        let rusqlite::Error::SqliteFailure(failure, message) = &error else {
            return Self::Database(error);
        };
        let message = message.clone().unwrap_or_default();
        match failure.extended_code {
            ffi::SQLITE_CONSTRAINT_UNIQUE | ffi::SQLITE_CONSTRAINT_PRIMARYKEY => {
                Self::Conflict(message)
            }
            ffi::SQLITE_CONSTRAINT_FOREIGNKEY => Self::NotFound("referenced item"),
            ffi::SQLITE_CONSTRAINT_TRIGGER | ffi::SQLITE_CONSTRAINT_CHECK => {
                Self::InvalidArgument(message)
            }
            _ => Self::Database(error),
        }
    }
}

impl From<NameError> for CatalogError {
    fn from(error: NameError) -> Self {
        Self::InvalidArgument(error.to_string())
    }
}

// Messages sent to the UI stay generic: no SQL, no paths.
impl From<CatalogError> for CoreError {
    fn from(error: CatalogError) -> Self {
        match error {
            CatalogError::NotFound(what) => {
                CoreError::new(ErrorCode::InvalidArgument, format!("Unknown {what}."))
            }
            CatalogError::Conflict(_) => {
                CoreError::new(ErrorCode::Conflict, "This item already exists.")
            }
            CatalogError::InvalidArgument(reason) => {
                CoreError::new(ErrorCode::InvalidArgument, reason)
            }
            CatalogError::Database(_) | CatalogError::SchemaTooNew { .. } => CoreError::new(
                ErrorCode::DatabaseError,
                "The catalog could not be read or written.",
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn failing(sql: &str) -> CatalogError {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             CREATE TABLE parent (id INTEGER PRIMARY KEY);
             CREATE TABLE child (
                 id INTEGER PRIMARY KEY,
                 parent INTEGER REFERENCES parent (id),
                 n INTEGER CHECK (n > 0)
             );
             INSERT INTO parent VALUES (1);",
        )
        .unwrap();
        conn.execute_batch(sql).unwrap_err().into()
    }

    #[test]
    fn classifies_unique_violations_as_conflicts() {
        assert!(matches!(
            failing("INSERT INTO parent VALUES (1);"),
            CatalogError::Conflict(_)
        ));
    }

    #[test]
    fn classifies_foreign_key_violations_as_not_found() {
        assert!(matches!(
            failing("INSERT INTO child VALUES (1, 99, 1);"),
            CatalogError::NotFound(_)
        ));
    }

    #[test]
    fn classifies_check_violations_as_invalid() {
        assert!(matches!(
            failing("INSERT INTO child VALUES (1, 1, 0);"),
            CatalogError::InvalidArgument(_)
        ));
    }

    #[test]
    fn keeps_other_errors_as_database_errors() {
        assert!(matches!(failing("NOT SQL"), CatalogError::Database(_)));
    }

    #[test]
    fn hides_internal_details_from_the_ui() {
        let error: CoreError = failing("NOT SQL").into();
        assert_eq!(error.code, ErrorCode::DatabaseError);
        assert!(!error.message.contains("SQL"));
    }
}
