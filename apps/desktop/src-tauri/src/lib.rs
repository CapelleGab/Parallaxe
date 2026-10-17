mod commands;
mod dto;
mod state;

use tauri::Manager;

use crate::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let state = AppState::open(&app.path().app_data_dir()?)?;
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::app::app_info,
            commands::library::library_create_with_folder,
            commands::library::library_list,
            commands::library::library_add_root,
            commands::library::root_list,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Parallaxe");
}
