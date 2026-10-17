use std::path::Path;

use parallaxe_core::id::{LibraryId, RootId};
use parallaxe_core::library::{Library, Root, RootStatus};
use parallaxe_core::{name, path};
use rusqlite::{Connection, OptionalExtension, Row, params};

use crate::rows::{now_ms, parse};
use crate::{Catalog, CatalogError};

impl Catalog {
    pub fn create_library(&self, raw_name: &str) -> Result<Library, CatalogError> {
        let library = new_library(raw_name)?;
        insert_library(self.conn(), &library)?;
        Ok(library)
    }

    /// Creates a library and its library folder together, or neither.
    pub fn create_library_with_folder(
        &mut self,
        raw_name: &str,
        folder: &Path,
    ) -> Result<(Library, Root), CatalogError> {
        let library = new_library(raw_name)?;
        let tx = self.conn_mut().transaction()?;
        insert_library(&tx, &library)?;
        let root = insert_root(&tx, library.id, folder, true)?;
        tx.commit()?;
        Ok((library, root))
    }

    pub fn list_libraries(&self) -> Result<Vec<Library>, CatalogError> {
        let mut statement = self.conn().prepare(
            "SELECT id, name, created_at, updated_at FROM libraries ORDER BY created_at, id",
        )?;
        let libraries = statement
            .query_map([], library_from_row)?
            .collect::<Result<_, _>>()?;
        Ok(libraries)
    }

    pub fn library(&self, id: LibraryId) -> Result<Library, CatalogError> {
        self.conn()
            .query_row(
                "SELECT id, name, created_at, updated_at FROM libraries WHERE id = ?1",
                [id.to_string()],
                library_from_row,
            )
            .optional()?
            .ok_or(CatalogError::NotFound("library"))
    }

    /// Registers an external folder, referenced in place.
    pub fn add_root(
        &mut self,
        library_id: LibraryId,
        root_path: &Path,
    ) -> Result<Root, CatalogError> {
        let tx = self.conn_mut().transaction()?;
        let library_exists = tx
            .query_row(
                "SELECT 1 FROM libraries WHERE id = ?1",
                [library_id.to_string()],
                |_| Ok(()),
            )
            .optional()?
            .is_some();
        if !library_exists {
            return Err(CatalogError::NotFound("library"));
        }
        let root = insert_root(&tx, library_id, root_path, false)?;
        tx.commit()?;
        Ok(root)
    }

    /// The library folder first, then external folders in the order they were added.
    pub fn list_roots(&self, library_id: LibraryId) -> Result<Vec<Root>, CatalogError> {
        let mut statement = self.conn().prepare(
            "SELECT id, library_id, path, status, is_primary, last_seen_at, created_at
             FROM roots WHERE library_id = ?1 ORDER BY is_primary DESC, created_at, id",
        )?;
        let roots = statement
            .query_map([library_id.to_string()], root_from_row)?
            .collect::<Result<_, _>>()?;
        Ok(roots)
    }

    pub fn root(&self, id: RootId) -> Result<Root, CatalogError> {
        self.conn()
            .query_row(
                "SELECT id, library_id, path, status, is_primary, last_seen_at, created_at
                 FROM roots WHERE id = ?1",
                [id.to_string()],
                root_from_row,
            )
            .optional()?
            .ok_or(CatalogError::NotFound("root"))
    }

    /// Removes the folder from the catalog only. Files on disk are never touched.
    pub fn remove_root(&self, id: RootId) -> Result<(), CatalogError> {
        let removed = self
            .conn()
            .execute("DELETE FROM roots WHERE id = ?1", [id.to_string()])?;
        if removed == 0 {
            return Err(CatalogError::NotFound("root"));
        }
        Ok(())
    }
}

fn new_library(raw_name: &str) -> Result<Library, CatalogError> {
    let now = now_ms();
    Ok(Library {
        id: LibraryId::new(),
        name: name::clean(raw_name)?,
        created_at: now,
        updated_at: now,
    })
}

fn insert_library(conn: &Connection, library: &Library) -> Result<(), CatalogError> {
    conn.execute(
        "INSERT INTO libraries (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
        params![
            library.id.to_string(),
            library.name,
            library.created_at,
            library.updated_at
        ],
    )?;
    Ok(())
}

/// The path must be absolute and already canonicalized by the caller.
fn insert_root(
    conn: &Connection,
    library_id: LibraryId,
    root_path: &Path,
    is_primary: bool,
) -> Result<Root, CatalogError> {
    if !root_path.is_absolute() {
        return Err(CatalogError::InvalidArgument(
            "root path must be absolute".into(),
        ));
    }

    let existing: Vec<Vec<u8>> = conn
        .prepare("SELECT path FROM roots WHERE library_id = ?1")?
        .query_map([library_id.to_string()], |row| row.get(0))?
        .collect::<Result<_, _>>()?;
    if existing
        .into_iter()
        .any(|bytes| path::overlaps(&path::from_bytes(bytes), root_path))
    {
        return Err(CatalogError::Conflict(
            "folder overlaps a folder already in this library".into(),
        ));
    }

    let root = Root {
        id: RootId::new(),
        library_id,
        path: root_path.to_path_buf(),
        status: RootStatus::Available,
        is_primary,
        last_seen_at: None,
        created_at: now_ms(),
    };
    conn.execute(
        "INSERT INTO roots (id, library_id, path, status, is_primary, created_at)
         VALUES (?1, ?2, ?3, 'available', ?4, ?5)",
        params![
            root.id.to_string(),
            root.library_id.to_string(),
            path::to_bytes(&root.path),
            root.is_primary,
            root.created_at
        ],
    )?;
    Ok(root)
}

fn library_from_row(row: &Row<'_>) -> rusqlite::Result<Library> {
    Ok(Library {
        id: parse(row, 0)?,
        name: row.get(1)?,
        created_at: row.get(2)?,
        updated_at: row.get(3)?,
    })
}

fn root_from_row(row: &Row<'_>) -> rusqlite::Result<Root> {
    let status: String = row.get(3)?;
    Ok(Root {
        id: parse(row, 0)?,
        library_id: parse(row, 1)?,
        path: path::from_bytes(row.get(2)?),
        status: if status == "offline" {
            RootStatus::Offline
        } else {
            RootStatus::Available
        },
        is_primary: row.get(4)?,
        last_seen_at: row.get(5)?,
        created_at: row.get(6)?,
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn absolute(path: &str) -> PathBuf {
        std::env::temp_dir().join(path)
    }

    #[test]
    fn creates_and_lists_libraries_in_creation_order() {
        let catalog = Catalog::open_in_memory().unwrap();
        let first = catalog.create_library("Games").unwrap();
        let second = catalog.create_library("  Design   refs ").unwrap();

        assert_eq!(second.name, "Design refs");
        assert_eq!(catalog.list_libraries().unwrap(), vec![first, second]);
    }

    #[test]
    fn rejects_blank_library_names() {
        let catalog = Catalog::open_in_memory().unwrap();
        assert!(matches!(
            catalog.create_library("   "),
            Err(CatalogError::InvalidArgument(_))
        ));
    }

    #[test]
    fn finds_a_library_by_id() {
        let catalog = Catalog::open_in_memory().unwrap();
        let library = catalog.create_library("Games").unwrap();

        assert_eq!(catalog.library(library.id).unwrap(), library);
        assert!(matches!(
            catalog.library(LibraryId::new()),
            Err(CatalogError::NotFound("library"))
        ));
    }

    #[test]
    fn adds_and_lists_roots() {
        let mut catalog = Catalog::open_in_memory().unwrap();
        let library = catalog.create_library("Games").unwrap();
        let root = catalog.add_root(library.id, &absolute("textures")).unwrap();

        assert_eq!(root.status, RootStatus::Available);
        assert_eq!(catalog.list_roots(library.id).unwrap(), vec![root]);
    }

    #[test]
    fn creates_a_library_with_its_folder() {
        let mut catalog = Catalog::open_in_memory().unwrap();
        let (library, folder) = catalog
            .create_library_with_folder("Games", &absolute("vault"))
            .unwrap();

        assert!(folder.is_primary);
        assert_eq!(folder.library_id, library.id);
        assert_eq!(catalog.list_libraries().unwrap(), vec![library.clone()]);
        assert_eq!(catalog.list_roots(library.id).unwrap(), vec![folder]);
    }

    #[test]
    fn creating_a_library_with_a_bad_folder_creates_nothing() {
        let mut catalog = Catalog::open_in_memory().unwrap();

        assert!(
            catalog
                .create_library_with_folder("Games", Path::new("relative"))
                .is_err()
        );
        assert!(catalog.list_libraries().unwrap().is_empty());
    }

    #[test]
    fn lists_the_library_folder_before_external_folders() {
        let mut catalog = Catalog::open_in_memory().unwrap();
        let (library, _) = catalog
            .create_library_with_folder("Games", &absolute("vault"))
            .unwrap();
        let external = catalog
            .add_root(library.id, &absolute("godot-project"))
            .unwrap();

        let roots = catalog.list_roots(library.id).unwrap();
        assert!(roots[0].is_primary);
        assert_eq!(roots[1], external);
        assert!(!external.is_primary);
    }

    #[test]
    fn allows_a_single_library_folder() {
        let mut catalog = Catalog::open_in_memory().unwrap();
        let (library, _) = catalog
            .create_library_with_folder("Games", &absolute("vault"))
            .unwrap();
        let external = catalog.add_root(library.id, &absolute("other")).unwrap();

        let result = catalog.conn().execute(
            "UPDATE roots SET is_primary = 1 WHERE id = ?1",
            [external.id.to_string()],
        );
        assert!(
            CatalogError::from(result.unwrap_err())
                .to_string()
                .contains("conflict")
        );
    }

    #[test]
    fn finds_a_root_by_id() {
        let mut catalog = Catalog::open_in_memory().unwrap();
        let (_, folder) = catalog
            .create_library_with_folder("Games", &absolute("vault"))
            .unwrap();

        assert_eq!(catalog.root(folder.id).unwrap(), folder);
        assert!(matches!(
            catalog.root(RootId::new()),
            Err(CatalogError::NotFound("root"))
        ));
    }

    #[test]
    fn rejects_relative_paths() {
        let mut catalog = Catalog::open_in_memory().unwrap();
        let library = catalog.create_library("Games").unwrap();

        assert!(matches!(
            catalog.add_root(library.id, Path::new("textures")),
            Err(CatalogError::InvalidArgument(_))
        ));
    }

    #[test]
    fn rejects_unknown_libraries() {
        let mut catalog = Catalog::open_in_memory().unwrap();
        assert!(matches!(
            catalog.add_root(LibraryId::new(), &absolute("textures")),
            Err(CatalogError::NotFound("library"))
        ));
    }

    #[test]
    fn rejects_duplicate_and_nested_roots() {
        let mut catalog = Catalog::open_in_memory().unwrap();
        let library = catalog.create_library("Games").unwrap();
        catalog.add_root(library.id, &absolute("assets")).unwrap();

        for path in ["assets", "assets/textures", ""] {
            assert!(matches!(
                catalog.add_root(library.id, &absolute(path)),
                Err(CatalogError::Conflict(_))
            ));
        }
        assert!(
            catalog
                .add_root(library.id, &absolute("assets-old"))
                .is_ok()
        );
    }

    #[test]
    fn the_same_folder_can_belong_to_two_libraries() {
        let mut catalog = Catalog::open_in_memory().unwrap();
        let games = catalog.create_library("Games").unwrap();
        let design = catalog.create_library("Design").unwrap();

        catalog.add_root(games.id, &absolute("shared")).unwrap();
        assert!(catalog.add_root(design.id, &absolute("shared")).is_ok());
    }

    #[test]
    fn removes_roots_and_reports_unknown_ones() {
        let mut catalog = Catalog::open_in_memory().unwrap();
        let library = catalog.create_library("Games").unwrap();
        let root = catalog.add_root(library.id, &absolute("textures")).unwrap();

        catalog.remove_root(root.id).unwrap();
        assert!(catalog.list_roots(library.id).unwrap().is_empty());
        assert!(matches!(
            catalog.remove_root(root.id),
            Err(CatalogError::NotFound("root"))
        ));
    }
}
