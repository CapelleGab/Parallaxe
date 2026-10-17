use parallaxe_core::id::{AssetId, CollectionId, LibraryId};
use parallaxe_core::library::Collection;
use parallaxe_core::name;
use rusqlite::{OptionalExtension, Row, params};

use crate::rows::{now_ms, parse};
use crate::{Catalog, CatalogError};

const COLLECTION_COLUMNS: &str =
    "c.id, c.library_id, c.name, c.description, c.created_at, c.updated_at";
const MAX_DESCRIPTION_CHARS: usize = 500;

impl Catalog {
    pub fn create_collection(
        &self,
        library_id: LibraryId,
        raw_name: &str,
        description: Option<&str>,
    ) -> Result<Collection, CatalogError> {
        let now = now_ms();
        let collection = Collection {
            id: CollectionId::new(),
            library_id,
            name: name::clean(raw_name)?,
            description: clean_description(description)?,
            created_at: now,
            updated_at: now,
        };
        self.conn().execute(
            "INSERT INTO collections (id, library_id, name, description, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                collection.id.to_string(),
                collection.library_id.to_string(),
                collection.name,
                collection.description,
                collection.created_at,
                collection.updated_at
            ],
        )?;
        Ok(collection)
    }

    pub fn list_collections(&self, library_id: LibraryId) -> Result<Vec<Collection>, CatalogError> {
        let mut statement = self.conn().prepare(&format!(
            "SELECT {COLLECTION_COLUMNS} FROM collections c WHERE c.library_id = ?1
             ORDER BY lower(c.name), c.id"
        ))?;
        let collections = statement
            .query_map([library_id.to_string()], collection_from_row)?
            .collect::<Result<_, _>>()?;
        Ok(collections)
    }

    /// Deletes the collection only: its assets stay in the catalog and on disk.
    pub fn delete_collection(&self, id: CollectionId) -> Result<(), CatalogError> {
        let deleted = self
            .conn()
            .execute("DELETE FROM collections WHERE id = ?1", [id.to_string()])?;
        if deleted == 0 {
            return Err(CatalogError::NotFound("collection"));
        }
        Ok(())
    }

    /// Adds every asset or none of them. Returns how many were not already in the collection.
    pub fn add_to_collection(
        &mut self,
        id: CollectionId,
        assets: &[AssetId],
    ) -> Result<usize, CatalogError> {
        self.collection(id)?;
        let now = now_ms();
        let tx = self.conn_mut().transaction()?;
        let mut added = 0;
        {
            let mut insert = tx.prepare(
                "INSERT OR IGNORE INTO collection_assets (collection_id, asset_id, added_at)
                 VALUES (?1, ?2, ?3)",
            )?;
            for asset in assets {
                added += insert.execute(params![id.to_string(), asset.to_string(), now])?;
            }
            tx.execute(
                "UPDATE collections SET updated_at = ?2 WHERE id = ?1",
                params![id.to_string(), now],
            )?;
        }
        tx.commit()?;
        Ok(added)
    }

    pub fn remove_from_collection(
        &mut self,
        id: CollectionId,
        assets: &[AssetId],
    ) -> Result<usize, CatalogError> {
        let tx = self.conn_mut().transaction()?;
        let mut removed = 0;
        {
            let mut delete = tx.prepare(
                "DELETE FROM collection_assets WHERE collection_id = ?1 AND asset_id = ?2",
            )?;
            for asset in assets {
                removed += delete.execute(params![id.to_string(), asset.to_string()])?;
            }
        }
        tx.commit()?;
        Ok(removed)
    }

    /// Assets in the order they were added.
    pub fn collection_assets(&self, id: CollectionId) -> Result<Vec<AssetId>, CatalogError> {
        self.collection(id)?;
        let mut statement = self.conn().prepare(
            "SELECT asset_id FROM collection_assets WHERE collection_id = ?1 ORDER BY added_at, asset_id",
        )?;
        let assets = statement
            .query_map([id.to_string()], |row| parse(row, 0))?
            .collect::<Result<_, _>>()?;
        Ok(assets)
    }

    fn collection(&self, id: CollectionId) -> Result<Collection, CatalogError> {
        self.conn()
            .query_row(
                &format!("SELECT {COLLECTION_COLUMNS} FROM collections c WHERE c.id = ?1"),
                [id.to_string()],
                collection_from_row,
            )
            .optional()?
            .ok_or(CatalogError::NotFound("collection"))
    }
}

fn clean_description(raw: Option<&str>) -> Result<Option<String>, CatalogError> {
    let Some(text) = raw.map(str::trim).filter(|text| !text.is_empty()) else {
        return Ok(None);
    };
    if text.chars().count() > MAX_DESCRIPTION_CHARS {
        return Err(CatalogError::InvalidArgument(format!(
            "description is longer than {MAX_DESCRIPTION_CHARS} characters"
        )));
    }
    Ok(Some(text.to_owned()))
}

fn collection_from_row(row: &Row<'_>) -> rusqlite::Result<Collection> {
    Ok(Collection {
        id: parse(row, 0)?,
        library_id: parse(row, 1)?,
        name: row.get(2)?,
        description: row.get(3)?,
        created_at: row.get(4)?,
        updated_at: row.get(5)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::Fixture;

    #[test]
    fn creates_collections_with_optional_description() {
        let f = Fixture::new();
        let castle = f
            .catalog
            .create_collection(f.library, "Castle", Some("  Medieval kit  "))
            .unwrap();
        let empty = f
            .catalog
            .create_collection(f.library, "Props", Some("   "))
            .unwrap();

        assert_eq!(castle.description.as_deref(), Some("Medieval kit"));
        assert_eq!(empty.description, None);
        assert_eq!(
            f.catalog.list_collections(f.library).unwrap(),
            vec![castle, empty]
        );
    }

    #[test]
    fn allows_two_collections_with_the_same_name() {
        let f = Fixture::new();
        f.catalog
            .create_collection(f.library, "Castle", None)
            .unwrap();
        assert!(
            f.catalog
                .create_collection(f.library, "Castle", None)
                .is_ok()
        );
    }

    #[test]
    fn rejects_too_long_descriptions() {
        let f = Fixture::new();
        let long = "x".repeat(MAX_DESCRIPTION_CHARS + 1);
        assert!(matches!(
            f.catalog
                .create_collection(f.library, "Castle", Some(&long)),
            Err(CatalogError::InvalidArgument(_))
        ));
    }

    #[test]
    fn an_asset_can_be_in_several_collections() {
        let mut f = Fixture::new();
        let a = f.asset(f.root, "a.png");
        let castle = f
            .catalog
            .create_collection(f.library, "Castle", None)
            .unwrap();
        let props = f
            .catalog
            .create_collection(f.library, "Props", None)
            .unwrap();

        assert_eq!(f.catalog.add_to_collection(castle.id, &[a]).unwrap(), 1);
        assert_eq!(f.catalog.add_to_collection(props.id, &[a]).unwrap(), 1);
        assert_eq!(f.catalog.add_to_collection(props.id, &[a]).unwrap(), 0);

        assert_eq!(f.catalog.collection_assets(castle.id).unwrap(), vec![a]);
        assert_eq!(f.catalog.collection_assets(props.id).unwrap(), vec![a]);
    }

    #[test]
    fn removes_assets_from_a_collection() {
        let mut f = Fixture::new();
        let a = f.asset(f.root, "a.png");
        let b = f.asset(f.root, "b.png");
        let castle = f
            .catalog
            .create_collection(f.library, "Castle", None)
            .unwrap();
        f.catalog.add_to_collection(castle.id, &[a, b]).unwrap();

        assert_eq!(
            f.catalog.remove_from_collection(castle.id, &[a]).unwrap(),
            1
        );
        assert_eq!(f.catalog.collection_assets(castle.id).unwrap(), vec![b]);
    }

    #[test]
    fn adding_is_all_or_nothing() {
        let mut f = Fixture::new();
        let a = f.asset(f.root, "a.png");
        let castle = f
            .catalog
            .create_collection(f.library, "Castle", None)
            .unwrap();

        assert!(
            f.catalog
                .add_to_collection(castle.id, &[a, AssetId::new()])
                .is_err()
        );
        assert!(f.catalog.collection_assets(castle.id).unwrap().is_empty());
    }

    #[test]
    fn refuses_assets_from_another_library() {
        let mut f = Fixture::new();
        let (_, other_root) = f.other_library();
        let foreign = f.asset(other_root, "x.png");
        let castle = f
            .catalog
            .create_collection(f.library, "Castle", None)
            .unwrap();

        assert!(matches!(
            f.catalog.add_to_collection(castle.id, &[foreign]),
            Err(CatalogError::InvalidArgument(_))
        ));
    }

    #[test]
    fn deleting_a_collection_keeps_its_assets() {
        let mut f = Fixture::new();
        let a = f.asset(f.root, "a.png");
        let castle = f
            .catalog
            .create_collection(f.library, "Castle", None)
            .unwrap();
        f.catalog.add_to_collection(castle.id, &[a]).unwrap();

        f.catalog.delete_collection(castle.id).unwrap();
        let assets: i64 = f
            .catalog
            .conn()
            .query_row("SELECT count(*) FROM assets", [], |row| row.get(0))
            .unwrap();
        assert_eq!(assets, 1);
        assert!(matches!(
            f.catalog.collection_assets(castle.id),
            Err(CatalogError::NotFound("collection"))
        ));
    }
}
