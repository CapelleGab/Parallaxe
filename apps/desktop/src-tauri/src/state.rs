use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use parallaxe_catalog::{Catalog, CatalogError};
use parallaxe_core::error::{CoreError, ErrorCode};
use parallaxe_core::id::RootId;

pub struct AppState {
    catalog: Arc<Mutex<Catalog>>,
    data_dir: PathBuf,
    scans: Arc<Mutex<HashMap<RootId, Arc<AtomicBool>>>>,
}

impl AppState {
    /// The catalog lives in the app data directory, never inside a user root.
    pub fn open(data_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        std::fs::create_dir_all(data_dir)?;
        let catalog = Catalog::open(data_dir.join("catalog.db"))?;
        Ok(Self::new(catalog, std::fs::canonicalize(data_dir)?))
    }

    fn new(catalog: Catalog, data_dir: PathBuf) -> Self {
        Self {
            catalog: Arc::new(Mutex::new(catalog)),
            data_dir,
            scans: Arc::default(),
        }
    }

    pub fn catalog(&self) -> Arc<Mutex<Catalog>> {
        Arc::clone(&self.catalog)
    }

    /// Folders a scan must never enter.
    pub fn excluded_dirs(&self) -> Vec<PathBuf> {
        vec![self.data_dir.clone()]
    }

    /// Registers a scan of `root`. Returns `None` if that root is already being scanned.
    pub fn register_scan(&self, root: RootId) -> Option<ScanRegistration> {
        let mut scans = lock(&self.scans);
        if scans.contains_key(&root) {
            return None;
        }
        let cancel = Arc::new(AtomicBool::new(false));
        scans.insert(root, Arc::clone(&cancel));
        Some(ScanRegistration {
            root,
            cancel,
            scans: Arc::clone(&self.scans),
        })
    }

    /// Asks a running scan to stop. Returns `false` when no scan is running for that root.
    pub fn cancel_scan(&self, root: RootId) -> bool {
        lock(&self.scans)
            .get(&root)
            .map(|cancel| cancel.store(true, Ordering::Relaxed))
            .is_some()
    }

    /// Runs a catalog operation on a blocking thread so the UI never waits on disk I/O.
    pub async fn with_catalog<T, F>(&self, operation: F) -> Result<T, CoreError>
    where
        T: Send + 'static,
        F: FnOnce(&mut Catalog) -> Result<T, CatalogError> + Send + 'static,
    {
        let catalog = Arc::clone(&self.catalog);
        tauri::async_runtime::spawn_blocking(move || {
            // a panic mid-operation rolled its transaction back, so the catalog is still usable
            operation(&mut lock(&catalog)).map_err(CoreError::from)
        })
        .await
        .map_err(|_| {
            CoreError::new(
                ErrorCode::DatabaseError,
                "The catalog could not be read or written.",
            )
        })?
    }
}

/// A panic mid-operation rolled its transaction back, so the data is still usable.
pub fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Keeps a scan registered while it runs; dropping it, even on panic, frees the root.
pub struct ScanRegistration {
    root: RootId,
    cancel: Arc<AtomicBool>,
    scans: Arc<Mutex<HashMap<RootId, Arc<AtomicBool>>>>,
}

impl ScanRegistration {
    pub fn cancel_flag(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.cancel)
    }
}

impl Drop for ScanRegistration {
    fn drop(&mut self) {
        lock(&self.scans).remove(&self.root);
    }
}

#[cfg(test)]
impl AppState {
    pub fn in_memory() -> Self {
        Self::new(Catalog::open_in_memory().unwrap(), std::env::temp_dir())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_the_catalog_in_the_data_directory() {
        let dir = tempfile::tempdir().unwrap();
        let data_dir = dir.path().join("nested").join("app");

        AppState::open(&data_dir).unwrap();
        assert!(data_dir.join("catalog.db").exists());
    }

    #[test]
    fn allows_one_scan_per_root_at_a_time() {
        let state = AppState::in_memory();
        let root = RootId::new();

        let registration = state.register_scan(root).unwrap();
        assert!(state.register_scan(root).is_none());
        assert!(state.register_scan(RootId::new()).is_some());

        drop(registration);
        assert!(state.register_scan(root).is_some());
    }

    #[test]
    fn cancels_only_running_scans() {
        let state = AppState::in_memory();
        let root = RootId::new();
        assert!(!state.cancel_scan(root));

        let registration = state.register_scan(root).unwrap();
        assert!(state.cancel_scan(root));
        assert!(registration.cancel_flag().load(Ordering::Relaxed));
    }

    #[test]
    fn runs_operations_and_maps_errors() {
        let state = AppState::in_memory();
        tauri::async_runtime::block_on(async {
            let library = state
                .with_catalog(|catalog| catalog.create_library("Games"))
                .await
                .unwrap();
            assert_eq!(library.name, "Games");

            let error = state
                .with_catalog(|catalog| catalog.create_library("  "))
                .await
                .unwrap_err();
            assert_eq!(error.code, ErrorCode::InvalidArgument);
        });
    }
}
