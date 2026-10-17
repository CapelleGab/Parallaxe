use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

use parallaxe_catalog::{Catalog, CatalogError};
use parallaxe_core::asset::ScannedFile;
use parallaxe_core::id::RootId;
use parallaxe_indexer::{ScanError, ScanItem};

use crate::state::lock;

/// Files are written in batches: fewer transactions, and the catalog lock is released between them.
const BATCH_SIZE: usize = 500;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ScanCounts {
    pub files: u64,
    pub skipped: u64,
    pub unreadable: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanEnd {
    Completed {
        counts: ScanCounts,
        missing: usize,
    },
    Cancelled {
        counts: ScanCounts,
    },
    /// The folder is unreachable, e.g. an unplugged drive. Nothing was purged.
    Offline,
}

/// Scans one root into the catalog. Runs on a blocking thread; `cancel` is checked between files.
pub fn run_scan(
    catalog: &Mutex<Catalog>,
    root_id: RootId,
    excluded: &[std::path::PathBuf],
    cancel: &AtomicBool,
    mut on_progress: impl FnMut(ScanCounts),
) -> Result<ScanEnd, CatalogError> {
    let (root_path, session) = {
        let catalog = lock(catalog);
        (catalog.root(root_id)?.path, catalog.begin_scan(root_id)?)
    };

    let files = match parallaxe_indexer::scan(&root_path, excluded) {
        Ok(files) => files,
        Err(ScanError::RootUnavailable) => {
            lock(catalog).mark_root_offline(root_id)?;
            return Ok(ScanEnd::Offline);
        }
    };

    let mut counts = ScanCounts::default();
    let mut batch: Vec<ScannedFile> = Vec::with_capacity(BATCH_SIZE);
    for item in files {
        if cancel.load(Ordering::Relaxed) {
            return Ok(ScanEnd::Cancelled { counts });
        }
        match item {
            ScanItem::File(file) => {
                counts.files += 1;
                batch.push(file);
            }
            ScanItem::Skipped { .. } => counts.skipped += 1,
            ScanItem::Unreadable { .. } => counts.unreadable += 1,
        }
        if batch.len() == BATCH_SIZE {
            lock(catalog).record_files(&session, &batch)?;
            batch.clear();
            on_progress(counts);
        }
    }

    let mut catalog = lock(catalog);
    catalog.record_files(&session, &batch)?;
    let outcome = catalog.finish_scan(session)?;
    on_progress(counts);
    Ok(ScanEnd::Completed {
        counts,
        missing: outcome.missing,
    })
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use parallaxe_core::asset::AssetStatus;
    use parallaxe_core::id::LibraryId;

    use super::*;

    struct Setup {
        catalog: Mutex<Catalog>,
        library: LibraryId,
        root: RootId,
        dir: tempfile::TempDir,
    }

    fn setup() -> Setup {
        let dir = tempfile::tempdir().unwrap();
        let mut catalog = Catalog::open_in_memory().unwrap();
        let (library, root) = catalog
            .create_library_with_folder("Games", &fs::canonicalize(dir.path()).unwrap())
            .unwrap();
        Setup {
            catalog: Mutex::new(catalog),
            library: library.id,
            root: root.id,
            dir,
        }
    }

    fn write(dir: &Path, name: &str) {
        let path = dir.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, "x").unwrap();
    }

    fn run(s: &Setup) -> ScanEnd {
        run_scan(&s.catalog, s.root, &[], &AtomicBool::new(false), |_| {}).unwrap()
    }

    #[test]
    fn indexes_every_file_and_reports_progress() {
        let s = setup();
        for i in 0..1_200 {
            write(s.dir.path(), &format!("textures/{i}.png"));
        }
        let mut reports = Vec::new();

        let end = run_scan(&s.catalog, s.root, &[], &AtomicBool::new(false), |counts| {
            reports.push(counts.files)
        })
        .unwrap();

        assert_eq!(
            end,
            ScanEnd::Completed {
                counts: ScanCounts {
                    files: 1_200,
                    skipped: 0,
                    unreadable: 0
                },
                missing: 0
            }
        );
        assert_eq!(reports, [500, 1_000, 1_200]);
        assert_eq!(lock(&s.catalog).count_assets(s.library).unwrap(), 1_200);
    }

    #[test]
    fn flags_deleted_files_on_the_next_scan() {
        let s = setup();
        write(s.dir.path(), "oak.png");
        write(s.dir.path(), "pine.png");
        run(&s);

        fs::remove_file(s.dir.path().join("pine.png")).unwrap();
        let end = run(&s);

        assert!(matches!(end, ScanEnd::Completed { missing: 1, .. }));
        let statuses: Vec<AssetStatus> = lock(&s.catalog)
            .list_assets(s.library, 0, 10)
            .unwrap()
            .into_iter()
            .map(|asset| asset.status)
            .collect();
        assert_eq!(statuses, [AssetStatus::Available, AssetStatus::Missing]);
    }

    #[test]
    fn stops_when_cancelled_and_marks_nothing_missing() {
        let s = setup();
        write(s.dir.path(), "oak.png");
        run(&s);

        let end = run_scan(&s.catalog, s.root, &[], &AtomicBool::new(true), |_| {}).unwrap();

        assert!(matches!(end, ScanEnd::Cancelled { .. }));
        let asset = &lock(&s.catalog).list_assets(s.library, 0, 1).unwrap()[0];
        assert_eq!(asset.status, AssetStatus::Available);
    }

    #[test]
    fn an_unreachable_folder_goes_offline_without_purging() {
        let s = setup();
        write(s.dir.path(), "oak.png");
        run(&s);

        let Setup {
            catalog,
            library,
            root,
            dir,
        } = s;
        drop(dir);
        let end = run_scan(&catalog, root, &[], &AtomicBool::new(false), |_| {}).unwrap();

        assert_eq!(end, ScanEnd::Offline);
        let asset = &lock(&catalog).list_assets(library, 0, 1).unwrap()[0];
        assert_eq!(asset.status, AssetStatus::RootOffline);
    }

    #[test]
    fn never_enters_excluded_folders() {
        let s = setup();
        write(s.dir.path(), "data/catalog.db");
        write(s.dir.path(), "oak.png");
        let excluded = [fs::canonicalize(s.dir.path()).unwrap().join("data")];

        run_scan(
            &s.catalog,
            s.root,
            &excluded,
            &AtomicBool::new(false),
            |_| {},
        )
        .unwrap();

        assert_eq!(lock(&s.catalog).count_assets(s.library).unwrap(), 1);
    }

    #[test]
    fn unknown_roots_are_an_error() {
        let s = setup();
        let result = run_scan(
            &s.catalog,
            RootId::new(),
            &[],
            &AtomicBool::new(false),
            |_| {},
        );
        assert!(matches!(result, Err(CatalogError::NotFound("root"))));
    }
}
