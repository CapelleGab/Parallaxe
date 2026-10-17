use std::path::Path;
use std::sync::{Arc, Mutex, PoisonError};

use parallaxe_catalog::{Catalog, CatalogError};
use parallaxe_core::error::{CoreError, ErrorCode};

pub struct AppState {
    catalog: Arc<Mutex<Catalog>>,
}

impl AppState {
    /// The catalog lives in the app data directory, never inside a user root.
    pub fn open(data_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        std::fs::create_dir_all(data_dir)?;
        let catalog = Catalog::open(data_dir.join("catalog.db"))?;
        Ok(Self::new(catalog))
    }

    fn new(catalog: Catalog) -> Self {
        Self {
            catalog: Arc::new(Mutex::new(catalog)),
        }
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
            let mut catalog = catalog.lock().unwrap_or_else(PoisonError::into_inner);
            operation(&mut catalog).map_err(CoreError::from)
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

#[cfg(test)]
impl AppState {
    pub fn in_memory() -> Self {
        Self::new(Catalog::open_in_memory().unwrap())
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
