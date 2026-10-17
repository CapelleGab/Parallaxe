mod commands;
mod dto;
mod scan;
mod state;
mod thumbnails;

use tauri::Manager;

use crate::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let state = AppState::open(&app.path().app_data_dir()?, &app.path().app_cache_dir()?)?;
            app.manage(state);
            Ok(())
        })
        .register_asynchronous_uri_scheme_protocol("thumb", |ctx, request, responder| {
            let app = ctx.app_handle().clone();
            let asset_id = request.uri().path().trim_start_matches('/').to_owned();
            tauri::async_runtime::spawn(async move {
                let thumbnail = app.state::<AppState>().thumbnail(asset_id).await;
                responder.respond(thumbnails::response(thumbnail));
            });
        })
        .invoke_handler(tauri::generate_handler![
            commands::app::app_info,
            commands::library::library_create_with_folder,
            commands::library::library_list,
            commands::library::library_add_root,
            commands::library::root_list,
            commands::scan::scan_start,
            commands::scan::scan_cancel,
            commands::scan::asset_list,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Parallaxe");
}
