use tauri::Manager;

pub mod db;
mod commands;
pub mod llm;

use db::Database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
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
            commands::generate_dream_title,
            commands::optimize_dream_description,
            commands::generate_dream_analysis,
            commands::get_dream_analysis_with_cards,
            commands::create_bug,
            commands::get_bug,
            commands::list_bugs,
            commands::update_bug,
            commands::delete_bug,
            commands::optimize_bug_description,
            commands::generate_bug_title,
            commands::comment_on_card,
            commands::comment_on_multiple_cards,
            commands::chat_with_history,
            commands::create_mind_dump,
            commands::get_mind_dump,
            commands::list_mind_dumps,
            commands::update_mind_dump,
            commands::delete_mind_dump,
            commands::search_mind_dumps,
            commands::get_card,
            commands::get_card_by_name,
            commands::list_cards,
            commands::list_cards_by_usage,
            commands::create_bug_with_cards,
            commands::link_card_to_bug,
            commands::get_bug_cards,
            commands::unlink_card_from_bug,
            commands::clear_bug_cards,
            commands::get_card_bugs,
            commands::backup_database,
            commands::get_database_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
