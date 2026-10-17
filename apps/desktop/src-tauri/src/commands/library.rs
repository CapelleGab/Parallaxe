use parallaxe_core::error::{CoreError, ErrorCode};
use parallaxe_core::id::LibraryId;
use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;

use super::parse_id;
use crate::dto::{LibraryDto, RootDto};
use crate::state::AppState;

#[tauri::command]
pub async fn library_create(
    state: State<'_, AppState>,
    name: String,
) -> Result<LibraryDto, CoreError> {
    let library = state
        .with_catalog(move |catalog| catalog.create_library(&name))
        .await?;
    Ok(library.into())
}

#[tauri::command]
pub async fn library_list(state: State<'_, AppState>) -> Result<Vec<LibraryDto>, CoreError> {
    let libraries = state
        .with_catalog(|catalog| catalog.list_libraries())
        .await?;
    Ok(libraries.into_iter().map(Into::into).collect())
}

#[tauri::command]
pub async fn root_list(
    state: State<'_, AppState>,
    library_id: String,
) -> Result<Vec<RootDto>, CoreError> {
    let library_id: LibraryId = parse_id(&library_id, "library")?;
    let roots = state
        .with_catalog(move |catalog| catalog.list_roots(library_id))
        .await?;
    Ok(roots.into_iter().map(Into::into).collect())
}

/// Opens the native folder picker. The WebView never sends a path: Rust gets it from the OS.
/// Returns `None` when the user cancels.
#[tauri::command]
pub async fn library_add_root(
    app: AppHandle,
    state: State<'_, AppState>,
    library_id: String,
) -> Result<Option<RootDto>, CoreError> {
    let library_id: LibraryId = parse_id(&library_id, "library")?;

    let dialog = app.dialog().file().set_title("Add a folder");
    let picked = tauri::async_runtime::spawn_blocking(move || dialog.blocking_pick_folder())
        .await
        .map_err(|_| inaccessible())?;
    let Some(folder) = picked else {
        return Ok(None);
    };

    let path = folder
        .into_path()
        .and_then(|path| Ok(std::fs::canonicalize(path)?))
        .map_err(|_| inaccessible())?;
    let root = state
        .with_catalog(move |catalog| catalog.add_root(library_id, &path))
        .await?;
    Ok(Some(root.into()))
}

fn inaccessible() -> CoreError {
    CoreError::new(
        ErrorCode::PermissionDenied,
        "This folder is not accessible. Allow access or choose another one.",
    )
}
