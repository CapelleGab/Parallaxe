use parallaxe_core::error::{CoreError, ErrorCode};
use parallaxe_core::id::{LibraryId, RootId};
use tauri::State;
use tauri::ipc::Channel;

use super::parse_id;
use crate::dto::{AssetPageDto, ScanProgressDto, ScanResultDto};
use crate::scan::run_scan;
use crate::state::AppState;

const MAX_PAGE_SIZE: u32 = 500;

/// Scans a root in the background. Progress is streamed on `on_progress`;
/// the command resolves once the scan is completed, cancelled or found offline.
#[tauri::command]
pub async fn scan_start(
    state: State<'_, AppState>,
    root_id: String,
    on_progress: Channel<ScanProgressDto>,
) -> Result<ScanResultDto, CoreError> {
    let root_id: RootId = parse_id(&root_id, "folder")?;
    let registration = state.register_scan(root_id).ok_or_else(|| {
        CoreError::new(ErrorCode::Conflict, "This folder is already being scanned.")
    })?;
    let catalog = state.catalog();
    let excluded = state.excluded_dirs();

    let end = tauri::async_runtime::spawn_blocking(move || {
        let cancel = registration.cancel_flag();
        run_scan(&catalog, root_id, &excluded, &cancel, |counts| {
            // a closed window simply stops listening; the scan itself goes on
            let _ = on_progress.send(counts.into());
        })
    })
    .await
    .map_err(|_| CoreError::new(ErrorCode::DatabaseError, "The scan stopped unexpectedly."))??;
    Ok(end.into())
}

#[tauri::command]
pub fn scan_cancel(state: State<'_, AppState>, root_id: String) -> Result<bool, CoreError> {
    let root_id: RootId = parse_id(&root_id, "folder")?;
    Ok(state.cancel_scan(root_id))
}

#[tauri::command]
pub async fn asset_list(
    state: State<'_, AppState>,
    library_id: String,
    offset: u32,
    limit: u32,
) -> Result<AssetPageDto, CoreError> {
    let library_id: LibraryId = parse_id(&library_id, "library")?;
    let limit = limit.min(MAX_PAGE_SIZE) as usize;
    let offset = offset as usize;
    state
        .with_catalog(move |catalog| {
            Ok(AssetPageDto {
                total: catalog.count_assets(library_id)?,
                items: catalog
                    .list_assets(library_id, offset, limit)?
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            })
        })
        .await
}
