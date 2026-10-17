use std::path::PathBuf;

use parallaxe_core::error::{CoreError, ErrorCode};
use parallaxe_core::id::LibraryId;
use parallaxe_core::name;
use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;

use super::parse_id;
use crate::dto::{LibraryDto, RootDto};
use crate::state::AppState;

/// Asks for the library folder, then creates the library and its folder together.
/// Returns `None` when the user cancels the folder picker.
#[tauri::command]
pub async fn library_create_with_folder(
    app: AppHandle,
    state: State<'_, AppState>,
    name: String,
) -> Result<Option<LibraryDto>, CoreError> {
    // validate before opening the picker, so a bad name never costs a folder choice
    name::clean(&name)
        .map_err(|error| CoreError::new(ErrorCode::InvalidArgument, error.to_string()))?;

    let Some(folder) = pick_folder(&app, "Choose or create your library folder").await? else {
        return Ok(None);
    };
    let (library, _) = state
        .with_catalog(move |catalog| catalog.create_library_with_folder(&name, &folder))
        .await?;
    Ok(Some(library.into()))
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

/// Adds an external folder, referenced in place. Returns `None` when the user cancels.
#[tauri::command]
pub async fn library_add_root(
    app: AppHandle,
    state: State<'_, AppState>,
    library_id: String,
) -> Result<Option<RootDto>, CoreError> {
    let library_id: LibraryId = parse_id(&library_id, "library")?;
    let Some(folder) = pick_folder(&app, "Add an external folder").await? else {
        return Ok(None);
    };
    let root = state
        .with_catalog(move |catalog| catalog.add_root(library_id, &folder))
        .await?;
    Ok(Some(root.into()))
}

/// The WebView never sends a path: Rust asks the OS, then resolves symlinks.
async fn pick_folder(app: &AppHandle, title: &str) -> Result<Option<PathBuf>, CoreError> {
    let dialog = app.dialog().file().set_title(title);
    let picked = tauri::async_runtime::spawn_blocking(move || dialog.blocking_pick_folder())
        .await
        .map_err(|_| inaccessible())?;
    let Some(folder) = picked else {
        return Ok(None);
    };
    folder
        .into_path()
        .and_then(|path| Ok(std::fs::canonicalize(path)?))
        .map(Some)
        .map_err(|_| inaccessible())
}

fn inaccessible() -> CoreError {
    CoreError::new(
        ErrorCode::PermissionDenied,
        "This folder is not accessible. Allow access or choose another one.",
    )
}
