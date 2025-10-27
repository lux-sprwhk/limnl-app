use tauri::Manager;

mod db;
mod commands;

use db::Database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Initialize database
            let db = Database::new().expect("Failed to initialize database");
            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::create_dream,
            commands::get_dream,
            commands::list_dreams,
            commands::update_dream,
            commands::delete_dream,
            commands::search_dreams,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
