use rusqlite::{Connection, params};

use crate::Catalog;

fn catalog() -> Catalog {
    let catalog = Catalog::open_in_memory().unwrap();
    let conn = catalog.conn();
    conn.execute_batch(
        "INSERT INTO libraries VALUES ('lib-a', 'Library A', 0, 0), ('lib-b', 'Library B', 0, 0);
         INSERT INTO roots (id, library_id, path, created_at) VALUES
             ('root-a', 'lib-a', x'2f612f', 0),
             ('root-b', 'lib-b', x'2f622f', 0);",
    )
    .unwrap();
    insert_asset(conn, "asset-a", "root-a", b"textures/wood.png").unwrap();
    insert_asset(conn, "asset-b", "root-b", b"sounds/door.wav").unwrap();
    catalog
}

fn insert_asset(
    conn: &Connection,
    id: &str,
    root: &str,
    rel_path: &[u8],
) -> rusqlite::Result<usize> {
    conn.execute(
        "INSERT INTO assets (id, root_id, rel_path, filename, kind, size_bytes, created_at, updated_at)
         VALUES (?1, ?2, ?3, 'file', 'image', 10, 0, 0)",
        params![id, root, rel_path],
    )
}

fn count(conn: &Connection, table: &str) -> i64 {
    conn.query_row(&format!("SELECT count(*) FROM {table}"), [], |row| {
        row.get(0)
    })
    .unwrap()
}

#[test]
fn stores_non_utf8_paths_exactly() {
    let catalog = catalog();
    let conn = catalog.conn();
    let raw = [b'b', b'a', b'd', 0xff, b'.', b'p', b'n', b'g'];
    insert_asset(conn, "asset-raw", "root-a", &raw).unwrap();

    let stored: Vec<u8> = conn
        .query_row(
            "SELECT rel_path FROM assets WHERE id = 'asset-raw'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(stored, raw);
}

#[test]
fn rejects_duplicate_paths_in_a_root() {
    let catalog = catalog();
    assert!(insert_asset(catalog.conn(), "dup", "root-a", b"textures/wood.png").is_err());
}

#[test]
fn rejects_unknown_status() {
    let catalog = catalog();
    let result = catalog.conn().execute(
        "UPDATE assets SET status = 'deleted' WHERE id = 'asset-a'",
        [],
    );
    assert!(result.is_err());
}

#[test]
fn rejects_asset_in_unknown_root() {
    let catalog = catalog();
    assert!(insert_asset(catalog.conn(), "orphan", "no-root", b"x.png").is_err());
}

#[test]
fn tag_names_are_unique_per_library() {
    let catalog = catalog();
    let conn = catalog.conn();
    let tag = "INSERT INTO tags (id, library_id, name, normalized_name, created_at) VALUES (?1, ?2, ?3, ?4, 0)";

    conn.execute(tag, params!["t1", "lib-a", "Wood", "wood"])
        .unwrap();
    assert!(
        conn.execute(tag, params!["t2", "lib-a", "WOOD", "wood"])
            .is_err()
    );
    conn.execute(tag, params!["t3", "lib-b", "wood", "wood"])
        .unwrap();
}

#[test]
fn rejects_blank_names() {
    let catalog = catalog();
    let result = catalog
        .conn()
        .execute("INSERT INTO libraries VALUES ('lib-c', '   ', 0, 0)", []);
    assert!(result.is_err());
}

#[test]
fn tags_cannot_cross_libraries() {
    let catalog = catalog();
    let conn = catalog.conn();
    conn.execute_batch(
        "INSERT INTO tags (id, library_id, name, normalized_name, created_at) VALUES ('tag-a', 'lib-a', 'wood', 'wood', 0);",
    )
    .unwrap();

    conn.execute("INSERT INTO asset_tags VALUES ('asset-a', 'tag-a')", [])
        .unwrap();
    assert!(
        conn.execute("INSERT INTO asset_tags VALUES ('asset-b', 'tag-a')", [])
            .is_err()
    );
}

#[test]
fn collections_cannot_cross_libraries() {
    let catalog = catalog();
    let conn = catalog.conn();
    conn.execute_batch(
        "INSERT INTO collections (id, library_id, name, created_at, updated_at) VALUES ('col-a', 'lib-a', 'Castle', 0, 0);",
    )
    .unwrap();

    conn.execute(
        "INSERT INTO collection_assets VALUES ('col-a', 'asset-a', 0)",
        [],
    )
    .unwrap();
    assert!(
        conn.execute(
            "INSERT INTO collection_assets VALUES ('col-a', 'asset-b', 0)",
            []
        )
        .is_err()
    );
}

#[test]
fn deleting_a_tag_keeps_assets() {
    let catalog = catalog();
    let conn = catalog.conn();
    conn.execute_batch(
        "INSERT INTO tags (id, library_id, name, normalized_name, created_at) VALUES ('tag-a', 'lib-a', 'wood', 'wood', 0);
         INSERT INTO asset_tags VALUES ('asset-a', 'tag-a');
         DELETE FROM tags WHERE id = 'tag-a';",
    )
    .unwrap();

    assert_eq!(count(conn, "asset_tags"), 0);
    assert_eq!(count(conn, "assets"), 2);
}

#[test]
fn removing_a_root_removes_its_catalog_entries_only() {
    let catalog = catalog();
    let conn = catalog.conn();
    conn.execute("DELETE FROM roots WHERE id = 'root-a'", [])
        .unwrap();

    let remaining: String = conn
        .query_row("SELECT id FROM assets", [], |row| row.get(0))
        .unwrap();
    assert_eq!(remaining, "asset-b");
}

#[test]
fn unknown_asset_is_a_foreign_key_error_not_a_library_mismatch() {
    let catalog = catalog();
    let conn = catalog.conn();
    conn.execute_batch(
        "INSERT INTO tags (id, library_id, name, normalized_name, created_at) VALUES ('tag-a', 'lib-a', 'wood', 'wood', 0);",
    )
    .unwrap();

    let error = conn
        .execute("INSERT INTO asset_tags VALUES ('ghost', 'tag-a')", [])
        .unwrap_err();
    assert_eq!(
        error.sqlite_error().map(|e| e.extended_code),
        Some(rusqlite::ffi::SQLITE_CONSTRAINT_FOREIGNKEY)
    );
}
