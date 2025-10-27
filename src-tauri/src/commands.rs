use crate::db::{models::*, Database};
use tauri::State;

#[tauri::command]
pub fn create_dream(
    db: State<Database>,
    input: CreateDreamInput,
) -> Result<Dream, String> {
    db.create_dream(input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_dream(
    db: State<Database>,
    id: i64,
) -> Result<Option<Dream>, String> {
    db.get_dream(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_dreams(
    db: State<Database>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Dream>, String> {
    db.list_dreams(limit, offset).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_dream(
    db: State<Database>,
    input: UpdateDreamInput,
) -> Result<Option<Dream>, String> {
    db.update_dream(input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_dream(
    db: State<Database>,
    id: i64,
) -> Result<bool, String> {
    db.delete_dream(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn search_dreams(
    db: State<Database>,
    query: String,
) -> Result<Vec<Dream>, String> {
    db.search_dreams(&query).map_err(|e| e.to_string())
}
