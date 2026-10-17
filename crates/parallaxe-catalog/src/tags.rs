use parallaxe_core::id::{AssetId, LibraryId, TagId};
use parallaxe_core::library::Tag;
use parallaxe_core::name;
use rusqlite::{OptionalExtension, Row, params};

use crate::rows::{now_ms, parse};
use crate::{Catalog, CatalogError};

const TAG_COLUMNS: &str = "t.id, t.library_id, t.name, t.color, t.created_at";

impl Catalog {
    pub fn create_tag(
        &self,
        library_id: LibraryId,
        raw_name: &str,
        color: Option<&str>,
    ) -> Result<Tag, CatalogError> {
        let tag = Tag {
            id: TagId::new(),
            library_id,
            name: name::clean(raw_name)?,
            color: color.map(validate_color).transpose()?,
            created_at: now_ms(),
        };
        self.conn().execute(
            "INSERT INTO tags (id, library_id, name, normalized_name, color, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                tag.id.to_string(),
                tag.library_id.to_string(),
                tag.name,
                name::normalize(&tag.name),
                tag.color,
                tag.created_at
            ],
        )?;
        Ok(tag)
    }

    pub fn list_tags(&self, library_id: LibraryId) -> Result<Vec<Tag>, CatalogError> {
        let mut statement = self.conn().prepare(&format!(
            "SELECT {TAG_COLUMNS} FROM tags t WHERE t.library_id = ?1 ORDER BY t.normalized_name"
        ))?;
        let tags = statement
            .query_map([library_id.to_string()], tag_from_row)?
            .collect::<Result<_, _>>()?;
        Ok(tags)
    }

    pub fn rename_tag(&self, id: TagId, raw_name: &str) -> Result<Tag, CatalogError> {
        let cleaned = name::clean(raw_name)?;
        let updated = self.conn().execute(
            "UPDATE tags SET name = ?2, normalized_name = ?3 WHERE id = ?1",
            params![id.to_string(), cleaned, name::normalize(&cleaned)],
        )?;
        if updated == 0 {
            return Err(CatalogError::NotFound("tag"));
        }
        self.tag(id)
    }

    /// Deletes the tag and its assignments. Assets and files are kept.
    pub fn delete_tag(&self, id: TagId) -> Result<(), CatalogError> {
        let deleted = self
            .conn()
            .execute("DELETE FROM tags WHERE id = ?1", [id.to_string()])?;
        if deleted == 0 {
            return Err(CatalogError::NotFound("tag"));
        }
        Ok(())
    }

    /// Tags every asset or none of them. Returns how many new assignments were made.
    pub fn assign_tag(&mut self, id: TagId, assets: &[AssetId]) -> Result<usize, CatalogError> {
        self.tag(id)?;
        let tx = self.conn_mut().transaction()?;
        let mut added = 0;
        {
            let mut insert =
                tx.prepare("INSERT OR IGNORE INTO asset_tags (asset_id, tag_id) VALUES (?1, ?2)")?;
            for asset in assets {
                added += insert.execute(params![asset.to_string(), id.to_string()])?;
            }
        }
        tx.commit()?;
        Ok(added)
    }

    pub fn unassign_tag(&mut self, id: TagId, assets: &[AssetId]) -> Result<usize, CatalogError> {
        let tx = self.conn_mut().transaction()?;
        let mut removed = 0;
        {
            let mut delete =
                tx.prepare("DELETE FROM asset_tags WHERE asset_id = ?1 AND tag_id = ?2")?;
            for asset in assets {
                removed += delete.execute(params![asset.to_string(), id.to_string()])?;
            }
        }
        tx.commit()?;
        Ok(removed)
    }

    pub fn tags_of_asset(&self, asset: AssetId) -> Result<Vec<Tag>, CatalogError> {
        let mut statement = self.conn().prepare(&format!(
            "SELECT {TAG_COLUMNS} FROM tags t JOIN asset_tags at ON at.tag_id = t.id
             WHERE at.asset_id = ?1 ORDER BY t.normalized_name"
        ))?;
        let tags = statement
            .query_map([asset.to_string()], tag_from_row)?
            .collect::<Result<_, _>>()?;
        Ok(tags)
    }

    fn tag(&self, id: TagId) -> Result<Tag, CatalogError> {
        self.conn()
            .query_row(
                &format!("SELECT {TAG_COLUMNS} FROM tags t WHERE t.id = ?1"),
                [id.to_string()],
                tag_from_row,
            )
            .optional()?
            .ok_or(CatalogError::NotFound("tag"))
    }
}

/// Colors are a visual hint only, stored as `#RRGGBB`.
fn validate_color(color: &str) -> Result<String, CatalogError> {
    let valid = color.len() == 7
        && color.starts_with('#')
        && color[1..].chars().all(|c| c.is_ascii_hexdigit());
    if valid {
        Ok(color.to_ascii_uppercase())
    } else {
        Err(CatalogError::InvalidArgument(
            "color must look like #RRGGBB".into(),
        ))
    }
}

fn tag_from_row(row: &Row<'_>) -> rusqlite::Result<Tag> {
    Ok(Tag {
        id: parse(row, 0)?,
        library_id: parse(row, 1)?,
        name: row.get(2)?,
        color: row.get(3)?,
        created_at: row.get(4)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::Fixture;

    #[test]
    fn creates_tags_with_clean_names() {
        let f = Fixture::new();
        let tag = f
            .catalog
            .create_tag(f.library, "  Dark   Wood ", Some("#a1b2c3"))
            .unwrap();

        assert_eq!(tag.name, "Dark Wood");
        assert_eq!(tag.color.as_deref(), Some("#A1B2C3"));
        assert_eq!(f.catalog.list_tags(f.library).unwrap(), vec![tag]);
    }

    #[test]
    fn rejects_invalid_colors() {
        let f = Fixture::new();
        for color in ["red", "#12345", "#GGGGGG", "123456#"] {
            assert!(matches!(
                f.catalog.create_tag(f.library, "wood", Some(color)),
                Err(CatalogError::InvalidArgument(_))
            ));
        }
    }

    #[test]
    fn rejects_duplicates_regardless_of_case() {
        let f = Fixture::new();
        f.catalog.create_tag(f.library, "Wood", None).unwrap();

        assert!(matches!(
            f.catalog.create_tag(f.library, "WOOD", None),
            Err(CatalogError::Conflict(_))
        ));
    }

    #[test]
    fn lists_tags_alphabetically_ignoring_case() {
        let f = Fixture::new();
        for name in ["stone", "Metal", "wood"] {
            f.catalog.create_tag(f.library, name, None).unwrap();
        }

        let names: Vec<String> = f
            .catalog
            .list_tags(f.library)
            .unwrap()
            .into_iter()
            .map(|tag| tag.name)
            .collect();
        assert_eq!(names, ["Metal", "stone", "wood"]);
    }

    #[test]
    fn renames_tags_and_checks_duplicates() {
        let f = Fixture::new();
        let wood = f.catalog.create_tag(f.library, "wood", None).unwrap();
        f.catalog.create_tag(f.library, "stone", None).unwrap();

        assert_eq!(f.catalog.rename_tag(wood.id, "Oak").unwrap().name, "Oak");
        assert!(matches!(
            f.catalog.rename_tag(wood.id, "Stone"),
            Err(CatalogError::Conflict(_))
        ));
        assert!(matches!(
            f.catalog.rename_tag(TagId::new(), "x"),
            Err(CatalogError::NotFound("tag"))
        ));
    }

    #[test]
    fn assigns_and_unassigns_in_bulk() {
        let mut f = Fixture::new();
        let a = f.asset(f.root, "a.png");
        let b = f.asset(f.root, "b.png");
        let tag = f.catalog.create_tag(f.library, "wood", None).unwrap();

        assert_eq!(f.catalog.assign_tag(tag.id, &[a, b]).unwrap(), 2);
        assert_eq!(f.catalog.assign_tag(tag.id, &[a]).unwrap(), 0);
        assert_eq!(f.catalog.tags_of_asset(a).unwrap(), vec![tag.clone()]);

        assert_eq!(f.catalog.unassign_tag(tag.id, &[a]).unwrap(), 1);
        assert!(f.catalog.tags_of_asset(a).unwrap().is_empty());
        assert_eq!(f.catalog.tags_of_asset(b).unwrap(), vec![tag]);
    }

    #[test]
    fn bulk_assignment_is_all_or_nothing() {
        let mut f = Fixture::new();
        let a = f.asset(f.root, "a.png");
        let tag = f.catalog.create_tag(f.library, "wood", None).unwrap();

        let result = f.catalog.assign_tag(tag.id, &[a, AssetId::new()]);
        assert!(matches!(result, Err(CatalogError::NotFound(_))));
        assert!(f.catalog.tags_of_asset(a).unwrap().is_empty());
    }

    #[test]
    fn refuses_assets_from_another_library() {
        let mut f = Fixture::new();
        let (_, other_root) = f.other_library();
        let foreign = f.asset(other_root, "x.png");
        let tag = f.catalog.create_tag(f.library, "wood", None).unwrap();

        assert!(matches!(
            f.catalog.assign_tag(tag.id, &[foreign]),
            Err(CatalogError::InvalidArgument(_))
        ));
    }

    #[test]
    fn reports_unknown_tags() {
        let mut f = Fixture::new();
        let a = f.asset(f.root, "a.png");
        assert!(matches!(
            f.catalog.assign_tag(TagId::new(), &[a]),
            Err(CatalogError::NotFound("tag"))
        ));
        assert!(matches!(
            f.catalog.delete_tag(TagId::new()),
            Err(CatalogError::NotFound("tag"))
        ));
    }

    #[test]
    fn deleting_a_tag_keeps_the_asset() {
        let mut f = Fixture::new();
        let a = f.asset(f.root, "a.png");
        let tag = f.catalog.create_tag(f.library, "wood", None).unwrap();
        f.catalog.assign_tag(tag.id, &[a]).unwrap();

        f.catalog.delete_tag(tag.id).unwrap();
        assert!(f.catalog.tags_of_asset(a).unwrap().is_empty());
        let assets: i64 = f
            .catalog
            .conn()
            .query_row("SELECT count(*) FROM assets", [], |row| row.get(0))
            .unwrap();
        assert_eq!(assets, 1);
    }
}
