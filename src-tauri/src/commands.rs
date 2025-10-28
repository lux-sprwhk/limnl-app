use crate::db::{models::*, Database};
use crate::llm::{
    client, GenerateTitleRequest, GenerateTitleResponse, OptimizeDescriptionRequest,
    OptimizeDescriptionResponse, CardCommentaryResponse,
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

// Bug LLM commands
#[tauri::command]
pub async fn optimize_bug_description(
    request: OptimizeDescriptionRequest,
) -> Result<OptimizeDescriptionResponse, String> {
    let optimized = client::optimize_description(&request.content, &request.config).await?;
    Ok(OptimizeDescriptionResponse { optimized })
}

#[tauri::command]
pub async fn generate_bug_title(
    request: GenerateTitleRequest,
) -> Result<GenerateTitleResponse, String> {
    let title = client::generate_title(&request.content, &request.config).await?;
    Ok(GenerateTitleResponse { title })
}

#[tauri::command]
pub async fn comment_on_card(
    request: serde_json::Value,
) -> Result<CardCommentaryResponse, String> {
    let card_name = request
        .get("card_name")
        .and_then(|v| v.as_str())
        .ok_or("Missing card_name")?;
    let card_question = request
        .get("card_question")
        .and_then(|v| v.as_str())
        .ok_or("Missing card_question")?;
    let card_meaning = request
        .get("card_meaning")
        .and_then(|v| v.as_str())
        .ok_or("Missing card_meaning")?;
    let life_area = request
        .get("life_area")
        .and_then(|v| v.as_str())
        .ok_or("Missing life_area")?;
    let config = serde_json::from_value(request.get("config").ok_or("Missing config")?.clone())
        .map_err(|e| format!("Invalid config: {}", e))?;

    let commentary = client::comment_on_card(card_name, card_question, card_meaning, life_area, &config).await?;
    Ok(CardCommentaryResponse { commentary })
}

#[tauri::command]
pub async fn comment_on_multiple_cards(
    request: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let cards = request
        .get("cards")
        .and_then(|v| v.as_array())
        .ok_or("Missing cards array")?;
    let life_area = request
        .get("life_area")
        .and_then(|v| v.as_str())
        .ok_or("Missing life_area")?;
    let config = serde_json::from_value(request.get("config").ok_or("Missing config")?.clone())
        .map_err(|e| format!("Invalid config: {}", e))?;

    let commentaries = client::comment_on_multiple_cards(cards, life_area, &config).await?;
    Ok(serde_json::json!({ "commentaries": commentaries }))
}

#[tauri::command]
pub async fn chat_with_history(
    request: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let user_message = request
        .get("user_message")
        .and_then(|v| v.as_str())
        .ok_or("Missing user_message")?;
    let messages = request
        .get("messages")
        .and_then(|v| v.as_array())
        .ok_or("Missing messages array")?;
    let card_name = request
        .get("card_name")
        .and_then(|v| v.as_str())
        .ok_or("Missing card_name")?;
    let card_question = request
        .get("card_question")
        .and_then(|v| v.as_str())
        .ok_or("Missing card_question")?;
    let card_meaning = request
        .get("card_meaning")
        .and_then(|v| v.as_str())
        .ok_or("Missing card_meaning")?;
    let card_insights = request
        .get("card_insights")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let life_area = request
        .get("life_area")
        .and_then(|v| v.as_str())
        .ok_or("Missing life_area")?;
    let config = serde_json::from_value(request.get("config").ok_or("Missing config")?.clone())
        .map_err(|e| format!("Invalid config: {}", e))?;

    let user_name = request
        .get("user_name")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let zodiac_sign = request
        .get("zodiac_sign")
        .and_then(|v| v.as_str());
    let mbti_type = request
        .get("mbti_type")
        .and_then(|v| v.as_str());

    let response = client::chat_with_history_with_profile(
        user_message,
        messages,
        card_name,
        card_question,
        card_meaning,
        card_insights,
        life_area,
        user_name,
        zodiac_sign,
        mbti_type,
        &config,
    )
    .await?;
    Ok(serde_json::json!({ "response": response }))
}

// Mind dump commands
#[tauri::command]
pub fn create_mind_dump(
    db: State<Database>,
    input: CreateMindDumpInput,
) -> Result<MindDump, String> {
    db.create_mind_dump(input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_mind_dump(
    db: State<Database>,
    id: i64,
) -> Result<Option<MindDump>, String> {
    db.get_mind_dump(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_mind_dumps(
    db: State<Database>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<MindDump>, String> {
    db.list_mind_dumps(limit, offset).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_mind_dump(
    db: State<Database>,
    input: UpdateMindDumpInput,
) -> Result<Option<MindDump>, String> {
    db.update_mind_dump(input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_mind_dump(
    db: State<Database>,
    id: i64,
) -> Result<bool, String> {
    db.delete_mind_dump(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn search_mind_dumps(
    db: State<Database>,
    query: String,
) -> Result<Vec<MindDump>, String> {
    db.search_mind_dumps(&query).map_err(|e| e.to_string())
}
