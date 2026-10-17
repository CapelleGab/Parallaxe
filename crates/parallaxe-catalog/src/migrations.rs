use rusqlite::Connection;

use crate::CatalogError;

const MIGRATIONS: &[&str] = &[include_str!("../migrations/0001_initial.sql")];

pub const SCHEMA_VERSION: usize = MIGRATIONS.len();

pub(crate) fn migrate(conn: &mut Connection) -> Result<(), CatalogError> {
    apply(conn, MIGRATIONS)
}

/// Applies every migration above the stored `user_version`, each in its own transaction.
fn apply(conn: &mut Connection, migrations: &[&str]) -> Result<(), CatalogError> {
    let current = schema_version(conn)?;
    if current > migrations.len() {
        return Err(CatalogError::SchemaTooNew {
            found: current,
            supported: migrations.len(),
        });
    }

    for (version, sql) in (1_i64..).zip(migrations).skip(current) {
        let tx = conn.transaction()?;
        tx.execute_batch(sql)?;
        tx.pragma_update(None, "user_version", version)?;
        tx.commit()?;
    }
    Ok(())
}

pub(crate) fn schema_version(conn: &Connection) -> Result<usize, CatalogError> {
    let version: i64 = conn.pragma_query_value(None, "user_version", |row| row.get(0))?;
    Ok(usize::try_from(version).unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FAKE: &[&str] = &[
        "CREATE TABLE a (id INTEGER PRIMARY KEY);",
        "CREATE TABLE b (id INTEGER PRIMARY KEY);",
    ];

    fn table_exists(conn: &Connection, name: &str) -> bool {
        conn.query_row(
            "SELECT count(*) FROM sqlite_schema WHERE type = 'table' AND name = ?1",
            [name],
            |row| row.get::<_, i64>(0),
        )
        .unwrap()
            == 1
    }

    #[test]
    fn applies_all_migrations_in_order() {
        let mut conn = Connection::open_in_memory().unwrap();
        apply(&mut conn, FAKE).unwrap();

        assert_eq!(schema_version(&conn).unwrap(), 2);
        assert!(table_exists(&conn, "a"));
        assert!(table_exists(&conn, "b"));
    }

    #[test]
    fn is_idempotent() {
        let mut conn = Connection::open_in_memory().unwrap();
        apply(&mut conn, FAKE).unwrap();
        apply(&mut conn, FAKE).unwrap();

        assert_eq!(schema_version(&conn).unwrap(), 2);
    }

    #[test]
    fn only_applies_new_migrations() {
        let mut conn = Connection::open_in_memory().unwrap();
        apply(&mut conn, &FAKE[..1]).unwrap();
        apply(&mut conn, FAKE).unwrap();

        assert_eq!(schema_version(&conn).unwrap(), 2);
    }

    #[test]
    fn failed_migration_rolls_back() {
        let mut conn = Connection::open_in_memory().unwrap();
        let broken = [
            "CREATE TABLE ok (id INTEGER);",
            "CREATE TABLE c (id INTEGER); NOT SQL;",
        ];

        assert!(apply(&mut conn, &broken).is_err());
        assert_eq!(schema_version(&conn).unwrap(), 1);
        assert!(!table_exists(&conn, "c"));
    }

    #[test]
    fn refuses_a_newer_schema() {
        let mut conn = Connection::open_in_memory().unwrap();
        apply(&mut conn, FAKE).unwrap();

        let error = apply(&mut conn, &FAKE[..1]).unwrap_err();
        assert!(matches!(
            error,
            CatalogError::SchemaTooNew {
                found: 2,
                supported: 1
            }
        ));
    }
}
