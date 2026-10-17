use std::path::{Component, PathBuf};

use parallaxe_core::asset::{Asset, ScannedFile};
use parallaxe_core::id::{AssetId, LibraryId, RootId};
use parallaxe_core::path;
use rusqlite::{OptionalExtension, Row, params};

use crate::rows::{now_ms, parse};
use crate::{Catalog, CatalogError};

/// An ongoing scan of one root. Dropping it without `finish_scan` (a cancelled scan)
/// leaves every asset as it was: nothing is marked missing.
#[derive(Debug)]
pub struct ScanSession {
    root_id: RootId,
    generation: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScanOutcome {
    /// Assets that were not found this time. Their tags and collections are kept.
    pub missing: usize,
}

/// An asset with the absolute path of its file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetSource {
    pub asset: Asset,
    pub path: PathBuf,
}

impl Catalog {
    /// Resolves an asset to its file. The path is guaranteed to stay inside its root.
    pub fn asset_source(&self, id: AssetId) -> Result<AssetSource, CatalogError> {
        let (asset, root_path) = self
            .conn()
            .query_row(
                "SELECT a.id, a.root_id, a.rel_path, a.filename, a.ext, a.kind, a.size_bytes,
                        a.mtime, a.status, a.is_favorite, r.path
                 FROM assets a JOIN roots r ON r.id = a.root_id WHERE a.id = ?1",
                [id.to_string()],
                |row| Ok((asset_from_row(row)?, path::from_bytes(row.get(10)?))),
            )
            .optional()?
            .ok_or(CatalogError::NotFound("asset"))?;

        let escapes = asset
            .rel_path
            .components()
            .any(|part| !matches!(part, Component::Normal(_)));
        if escapes {
            return Err(CatalogError::InvalidArgument(
                "asset path leaves its folder".into(),
            ));
        }
        Ok(AssetSource {
            path: root_path.join(&asset.rel_path),
            asset,
        })
    }

    pub fn begin_scan(&self, root_id: RootId) -> Result<ScanSession, CatalogError> {
        let generation = self
            .conn()
            .query_row(
                "UPDATE roots SET scan_generation = scan_generation + 1 WHERE id = ?1
                 RETURNING scan_generation",
                [root_id.to_string()],
                |row| row.get(0),
            )
            .optional()?
            .ok_or(CatalogError::NotFound("root"))?;
        Ok(ScanSession {
            root_id,
            generation,
        })
    }

    /// Inserts new files and refreshes known ones, in one transaction per batch.
    pub fn record_files(
        &mut self,
        session: &ScanSession,
        files: &[ScannedFile],
    ) -> Result<(), CatalogError> {
        let now = now_ms();
        let tx = self.conn_mut().transaction()?;
        {
            let mut upsert = tx.prepare(
                "INSERT INTO assets (id, root_id, rel_path, filename, ext, kind, size_bytes, mtime,
                                     status, seen_generation, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'available', ?9, ?10, ?10)
                 ON CONFLICT (root_id, rel_path) DO UPDATE SET
                     filename = excluded.filename,
                     ext = excluded.ext,
                     kind = excluded.kind,
                     size_bytes = excluded.size_bytes,
                     mtime = excluded.mtime,
                     status = 'available',
                     seen_generation = excluded.seen_generation,
                     updated_at = excluded.updated_at",
            )?;
            for file in files {
                upsert.execute(params![
                    AssetId::new().to_string(),
                    session.root_id.to_string(),
                    path::to_bytes(&file.rel_path),
                    file.filename,
                    file.ext,
                    file.kind.as_str(),
                    i64::try_from(file.size_bytes).unwrap_or(i64::MAX),
                    file.mtime_ms,
                    session.generation,
                    now,
                ])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    /// Completes a full scan: files not seen during it become `missing`, never deleted.
    pub fn finish_scan(&mut self, session: ScanSession) -> Result<ScanOutcome, CatalogError> {
        let now = now_ms();
        let tx = self.conn_mut().transaction()?;
        let missing = tx.execute(
            "UPDATE assets SET status = 'missing', updated_at = ?3
             WHERE root_id = ?1 AND seen_generation < ?2 AND status <> 'missing'",
            params![session.root_id.to_string(), session.generation, now],
        )?;
        tx.execute(
            "UPDATE roots SET status = 'available', last_seen_at = ?2 WHERE id = ?1",
            params![session.root_id.to_string(), now],
        )?;
        tx.commit()?;
        Ok(ScanOutcome { missing })
    }

    /// The root is unreachable (unplugged drive…): its assets are flagged, never purged.
    pub fn mark_root_offline(&mut self, root_id: RootId) -> Result<(), CatalogError> {
        let tx = self.conn_mut().transaction()?;
        let updated = tx.execute(
            "UPDATE roots SET status = 'offline' WHERE id = ?1",
            [root_id.to_string()],
        )?;
        if updated == 0 {
            return Err(CatalogError::NotFound("root"));
        }
        tx.execute(
            "UPDATE assets SET status = 'root_offline', updated_at = ?2
             WHERE root_id = ?1 AND status = 'available'",
            params![root_id.to_string(), now_ms()],
        )?;
        tx.commit()?;
        Ok(())
    }

    /// One page of a library's assets, sorted by name.
    pub fn list_assets(
        &self,
        library_id: LibraryId,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<Asset>, CatalogError> {
        let mut statement = self.conn().prepare(
            "SELECT a.id, a.root_id, a.rel_path, a.filename, a.ext, a.kind, a.size_bytes,
                    a.mtime, a.status, a.is_favorite
             FROM assets a JOIN roots r ON r.id = a.root_id
             WHERE r.library_id = ?1
             ORDER BY a.filename COLLATE NOCASE, a.id
             LIMIT ?2 OFFSET ?3",
        )?;
        let assets = statement
            .query_map(
                params![
                    library_id.to_string(),
                    i64::try_from(limit).unwrap_or(i64::MAX),
                    i64::try_from(offset).unwrap_or(i64::MAX)
                ],
                asset_from_row,
            )?
            .collect::<Result<_, _>>()?;
        Ok(assets)
    }

    pub fn count_assets(&self, library_id: LibraryId) -> Result<usize, CatalogError> {
        let count: i64 = self.conn().query_row(
            "SELECT count(*) FROM assets a JOIN roots r ON r.id = a.root_id WHERE r.library_id = ?1",
            [library_id.to_string()],
            |row| row.get(0),
        )?;
        Ok(usize::try_from(count).unwrap_or(0))
    }
}

fn asset_from_row(row: &Row<'_>) -> rusqlite::Result<Asset> {
    let size: i64 = row.get(6)?;
    Ok(Asset {
        id: parse(row, 0)?,
        root_id: parse(row, 1)?,
        rel_path: path::from_bytes(row.get(2)?),
        filename: row.get(3)?,
        ext: row.get(4)?,
        kind: parse(row, 5)?,
        size_bytes: u64::try_from(size).unwrap_or(0),
        mtime_ms: row.get(7)?,
        status: parse(row, 8)?,
        is_favorite: row.get(9)?,
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use parallaxe_core::asset::{AssetKind, AssetStatus};

    use super::*;
    use crate::test_support::Fixture;

    fn file(rel_path: &str, size: u64) -> ScannedFile {
        let rel_path = PathBuf::from(rel_path);
        let ext = rel_path
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default();
        ScannedFile {
            filename: rel_path.file_name().unwrap().to_string_lossy().into_owned(),
            kind: AssetKind::from_extension(&ext),
            ext,
            size_bytes: size,
            mtime_ms: Some(1),
            rel_path,
        }
    }

    fn scan(f: &mut Fixture, files: &[ScannedFile]) -> ScanOutcome {
        let session = f.catalog.begin_scan(f.root).unwrap();
        f.catalog.record_files(&session, files).unwrap();
        f.catalog.finish_scan(session).unwrap()
    }

    fn statuses(f: &Fixture) -> Vec<(String, AssetStatus)> {
        f.catalog
            .list_assets(f.library, 0, 100)
            .unwrap()
            .into_iter()
            .map(|asset| (asset.filename, asset.status))
            .collect()
    }

    #[test]
    fn records_new_files() {
        let mut f = Fixture::new();
        scan(
            &mut f,
            &[file("textures/oak.png", 10), file("door.wav", 20)],
        );

        let assets = f.catalog.list_assets(f.library, 0, 10).unwrap();
        assert_eq!(assets.len(), 2);
        assert_eq!(assets[0].filename, "door.wav");
        assert_eq!(assets[0].kind, AssetKind::Audio);
        assert_eq!(assets[1].rel_path, PathBuf::from("textures/oak.png"));
        assert_eq!(assets[1].size_bytes, 10);
        assert_eq!(f.catalog.count_assets(f.library).unwrap(), 2);
    }

    #[test]
    fn rescanning_keeps_ids_and_refreshes_properties() {
        let mut f = Fixture::new();
        scan(&mut f, &[file("oak.png", 10)]);
        let before = f.catalog.list_assets(f.library, 0, 1).unwrap()[0].clone();

        scan(&mut f, &[file("oak.png", 99)]);
        let after = f.catalog.list_assets(f.library, 0, 1).unwrap()[0].clone();

        assert_eq!(after.id, before.id);
        assert_eq!(after.size_bytes, 99);
        assert_eq!(f.catalog.count_assets(f.library).unwrap(), 1);
    }

    #[test]
    fn files_gone_from_disk_become_missing_and_keep_their_tags() {
        let mut f = Fixture::new();
        scan(&mut f, &[file("oak.png", 1), file("pine.png", 1)]);
        let pine = f.catalog.list_assets(f.library, 0, 10).unwrap()[1].id;
        let tag = f.catalog.create_tag(f.library, "wood", None).unwrap();
        f.catalog.assign_tag(tag.id, &[pine]).unwrap();

        let outcome = scan(&mut f, &[file("oak.png", 1)]);

        assert_eq!(outcome.missing, 1);
        assert_eq!(
            statuses(&f),
            [
                ("oak.png".into(), AssetStatus::Available),
                ("pine.png".into(), AssetStatus::Missing)
            ]
        );
        assert_eq!(f.catalog.tags_of_asset(pine).unwrap(), vec![tag]);
    }

    #[test]
    fn a_file_that_comes_back_is_available_again() {
        let mut f = Fixture::new();
        scan(&mut f, &[file("oak.png", 1)]);
        scan(&mut f, &[]);
        let outcome = scan(&mut f, &[file("oak.png", 1)]);

        assert_eq!(outcome.missing, 0);
        assert_eq!(statuses(&f), [("oak.png".into(), AssetStatus::Available)]);
    }

    #[test]
    fn a_cancelled_scan_marks_nothing_missing() {
        let mut f = Fixture::new();
        scan(&mut f, &[file("oak.png", 1), file("pine.png", 1)]);

        {
            // the session ends here without finish_scan, like a cancelled scan
            let session = f.catalog.begin_scan(f.root).unwrap();
            f.catalog
                .record_files(&session, &[file("oak.png", 1)])
                .unwrap();
        }

        assert!(
            statuses(&f)
                .iter()
                .all(|(_, s)| *s == AssetStatus::Available)
        );
    }

    #[test]
    fn an_offline_root_flags_its_assets_without_purging_them() {
        let mut f = Fixture::new();
        scan(&mut f, &[file("oak.png", 1)]);

        f.catalog.mark_root_offline(f.root).unwrap();
        assert_eq!(statuses(&f), [("oak.png".into(), AssetStatus::RootOffline)]);
        assert_eq!(
            f.catalog.list_roots(f.library).unwrap()[0].status,
            parallaxe_core::library::RootStatus::Offline
        );

        scan(&mut f, &[file("oak.png", 1)]);
        assert_eq!(statuses(&f), [("oak.png".into(), AssetStatus::Available)]);
        assert_eq!(
            f.catalog.list_roots(f.library).unwrap()[0].status,
            parallaxe_core::library::RootStatus::Available
        );
    }

    #[test]
    fn records_large_batches() {
        let mut f = Fixture::new();
        let files: Vec<ScannedFile> = (0..5_000).map(|i| file(&format!("{i}.png"), 1)).collect();
        scan(&mut f, &files);

        assert_eq!(f.catalog.count_assets(f.library).unwrap(), 5_000);
        assert_eq!(
            f.catalog.list_assets(f.library, 4_990, 100).unwrap().len(),
            10
        );
    }

    #[test]
    fn resolves_an_asset_to_its_file() {
        let mut f = Fixture::new();
        scan(&mut f, &[file("textures/oak.png", 1)]);
        let id = f.catalog.list_assets(f.library, 0, 1).unwrap()[0].id;
        let root = f.catalog.root(f.root).unwrap();

        let source = f.catalog.asset_source(id).unwrap();
        assert_eq!(source.path, root.path.join("textures").join("oak.png"));
        assert_eq!(source.asset.id, id);
        assert!(matches!(
            f.catalog.asset_source(AssetId::new()),
            Err(CatalogError::NotFound("asset"))
        ));
    }

    #[test]
    fn refuses_paths_that_would_leave_the_root() {
        let mut f = Fixture::new();
        scan(&mut f, &[file("../../secret.png", 1)]);
        let id = f.catalog.list_assets(f.library, 0, 1).unwrap()[0].id;

        assert!(matches!(
            f.catalog.asset_source(id),
            Err(CatalogError::InvalidArgument(_))
        ));
    }

    #[test]
    fn unknown_roots_cannot_be_scanned() {
        let f = Fixture::new();
        assert!(matches!(
            f.catalog.begin_scan(RootId::new()),
            Err(CatalogError::NotFound("root"))
        ));
    }

    #[test]
    fn libraries_only_see_their_own_assets() {
        let mut f = Fixture::new();
        scan(&mut f, &[file("oak.png", 1)]);
        let (other, _) = f.other_library();

        assert!(f.catalog.list_assets(other, 0, 10).unwrap().is_empty());
        assert_eq!(f.catalog.count_assets(other).unwrap(), 0);
    }
}
