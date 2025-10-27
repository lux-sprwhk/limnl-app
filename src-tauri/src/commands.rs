use crate::db::{models::*, Database};
use crate::llm::{
    client, GenerateTitleRequest, GenerateTitleResponse, OptimizeDescriptionRequest,
    OptimizeDescriptionResponse,
};
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

#[tauri::command]
pub async fn generate_dream_title(
    request: GenerateTitleRequest,
) -> Result<GenerateTitleResponse, String> {
    let title = client::generate_title(&request.content, &request.config).await?;
    Ok(GenerateTitleResponse { title })
}

#[tauri::command]
pub async fn optimize_dream_description(
    request: OptimizeDescriptionRequest,
) -> Result<OptimizeDescriptionResponse, String> {
    let optimized = client::optimize_description(&request.content, &request.config).await?;
    Ok(OptimizeDescriptionResponse { optimized })
}

// Bug tracking commands
#[tauri::command]
pub fn create_bug(
    db: State<Database>,
    input: CreateBugInput,
) -> Result<Bug, String> {
    db.create_bug(input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_bug(
    db: State<Database>,
    id: i64,
) -> Result<Option<Bug>, String> {
    db.get_bug(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_bugs(
    db: State<Database>,
    status: Option<String>,
) -> Result<Vec<Bug>, String> {
    db.list_bugs(status).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_bug(
    db: State<Database>,
    input: UpdateBugInput,
) -> Result<Option<Bug>, String> {
    db.update_bug(input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_bug(
    db: State<Database>,
    id: i64,
) -> Result<bool, String> {
    db.delete_bug(id).map_err(|e| e.to_string())
}
