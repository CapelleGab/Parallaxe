use parallaxe_core::id::{AssetId, LibraryId, RootId};
use rusqlite::params;

use crate::Catalog;

/// A library with one root, until the scanner can create real assets.
pub(crate) struct Fixture {
    pub catalog: Catalog,
    pub library: LibraryId,
    pub root: RootId,
}

impl Fixture {
    pub fn new() -> Self {
        let mut catalog = Catalog::open_in_memory().unwrap();
        let library = catalog.create_library("Games").unwrap().id;
        let root = catalog
            .add_root(library, &std::env::temp_dir().join("fixture"))
            .unwrap()
            .id;
        Self {
            catalog,
            library,
            root,
        }
    }

    pub fn other_library(&mut self) -> (LibraryId, RootId) {
        let library = self.catalog.create_library("Other").unwrap().id;
        let root = self
            .catalog
            .add_root(library, &std::env::temp_dir().join("fixture-other"))
            .unwrap()
            .id;
        (library, root)
    }

    pub fn asset(&self, root: RootId, rel_path: &str) -> AssetId {
        let id = AssetId::new();
        self.catalog
            .conn()
            .execute(
                "INSERT INTO assets (id, root_id, rel_path, filename, kind, size_bytes, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, 'image', 1, 0, 0)",
                params![id.to_string(), root.to_string(), rel_path.as_bytes(), rel_path],
            )
            .unwrap();
        id
    }
}
