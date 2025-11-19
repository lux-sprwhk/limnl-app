use crate::db::{models::*, Database};
use crate::llm::{
    client, GenerateTitleRequest, GenerateTitleResponse, OptimizeDescriptionRequest,
    OptimizeDescriptionResponse, CardCommentaryResponse, GenerateDreamAnalysisRequest,
    GenerateCreativePromptsRequest, LLMConfig, GenerateMindDumpAnalysisResponse, LLMProvider,
};
use tauri::State;
use std::path::PathBuf;

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

// Dream analysis commands
#[tauri::command]
pub async fn generate_dream_analysis(
    db: State<'_, Database>,
    request: GenerateDreamAnalysisRequest,
) -> Result<DreamAnalysisWithCards, String> {
    // Call LLM to generate analysis
    let llm_response = client::generate_dream_analysis(
        &request.dream_title,
        &request.dream_content,
        request.sleep_quality,
        &request.config
    ).await?;

    // Create the analysis in the database
    let analysis_input = CreateDreamAnalysisInput {
        dream_id: request.dream_id,
        themes_patterns: llm_response.themes_patterns.clone(),
        emotional_analysis: llm_response.emotional_analysis.clone(),
        narrative_summary: llm_response.narrative_summary.clone(),
    };

    let analysis = db.create_dream_analysis(analysis_input)
        .map_err(|e| e.to_string())?;

    let analysis_id = analysis.id.ok_or("Analysis ID not found")?;

    // Link the identified cards to the analysis
    for symbol_card in &llm_response.symbol_cards {
        // Get the card by name
        let card = db.get_card_by_name(&symbol_card.card_name)
            .map_err(|e| e.to_string())?
            .ok_or(format!("Card '{}' not found", symbol_card.card_name))?;

        // Link the card to the analysis
        db.link_card_to_dream_analysis(
            analysis_id,
            card.id.ok_or("Card ID not found")?,
            Some(symbol_card.relevance_note.clone())
        ).map_err(|e| e.to_string())?;
    }

    // Return the complete analysis with cards
    db.get_dream_analysis_with_cards(analysis.dream_id)
        .map_err(|e| e.to_string())?
        .ok_or("Failed to retrieve analysis with cards".to_string())
}

#[tauri::command]
pub fn get_dream_analysis_with_cards(
    db: State<Database>,
    dream_id: i64,
) -> Result<Option<DreamAnalysisWithCards>, String> {
    db.get_dream_analysis_with_cards(dream_id).map_err(|e| e.to_string())
}

// Dream creative prompts commands
#[tauri::command]
pub async fn generate_dream_creative_prompts(
    db: State<'_, Database>,
    request: GenerateCreativePromptsRequest,
) -> Result<DreamCreativePrompts, String> {
    // Call LLM to generate creative prompts
    let llm_response = client::generate_creative_prompts(
        &request.themes_patterns,
        &request.emotional_analysis,
        &request.narrative_summary,
        &request.config
    ).await?;

    // Convert arrays to JSON strings
    let image_prompts_json = serde_json::to_string(&llm_response.image_prompts)
        .map_err(|e| format!("Failed to serialize image prompts: {}", e))?;
    let music_prompts_json = serde_json::to_string(&llm_response.music_prompts)
        .map_err(|e| format!("Failed to serialize music prompts: {}", e))?;
    let story_prompts_json = serde_json::to_string(&llm_response.story_prompts)
        .map_err(|e| format!("Failed to serialize story prompts: {}", e))?;

    // Create the creative prompts in the database
    let prompts_input = CreateDreamCreativePromptsInput {
        dream_analysis_id: request.dream_analysis_id,
        image_prompts: image_prompts_json,
        music_prompts: music_prompts_json,
        story_prompts: story_prompts_json,
    };

    db.create_dream_creative_prompts(prompts_input)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_dream_creative_prompts(
    db: State<Database>,
    dream_analysis_id: i64,
) -> Result<Option<DreamCreativePrompts>, String> {
    db.get_dream_creative_prompts(dream_analysis_id).map_err(|e| e.to_string())
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
    let empty_cards = vec![];
    let selected_cards = request
        .get("selected_cards")
        .and_then(|v| v.as_array())
        .unwrap_or(&empty_cards);
    let config = serde_json::from_value(request.get("config").ok_or("Missing config")?.clone())
        .map_err(|e| format!("Invalid config: {}", e))?;

    let commentary = client::comment_on_card_with_context(card_name, card_question, card_meaning, life_area, selected_cards, &config).await?;
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
    let empty_cards = vec![];
    let selected_cards = request
        .get("selected_cards")
        .and_then(|v| v.as_array())
        .unwrap_or(&empty_cards);
    let config = serde_json::from_value(request.get("config").ok_or("Missing config")?.clone())
        .map_err(|e| format!("Invalid config: {}", e))?;

    let commentaries = client::comment_on_multiple_cards_with_context(cards, life_area, selected_cards, &config).await?;
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
    let selected_cards = request
        .get("selected_cards")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

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
        &selected_cards,
        &config,
    )
    .await?;
    Ok(serde_json::json!({ "response": response }))
}

// Mind dump commands
#[tauri::command]
pub fn create_mind_dump(
    db: State<'_, Database>,
    input: CreateMindDumpInput,
    config: LLMConfig,
) -> Result<MindDump, String> {
    // Validate input - trim first, then validate
    let trimmed_content = input.content.trim();
    if trimmed_content.is_empty() {
        return Err("Content cannot be empty".to_string());
    }
    
    const MAX_CHARACTERS: i32 = 2000;
    if trimmed_content.len() > MAX_CHARACTERS as usize {
        return Err(format!("Content exceeds maximum length of {} characters", MAX_CHARACTERS));
    }
    
    // Use trimmed length as the source of truth for character_count
    // This prevents mismatch errors when frontend sends count based on untrimmed content
    let trimmed_character_count = trimmed_content.len() as i32;
    
    // Create the mind dump first (synchronous DB operation)
    // Use trimmed content and trimmed character count for consistency
    let mut validated_input = input.clone();
    validated_input.content = trimmed_content.to_string();
    validated_input.character_count = trimmed_character_count;
    let mind_dump = db.create_mind_dump(validated_input).map_err(|e| e.to_string())?;
    let mind_dump_id = mind_dump.id.ok_or("Mind dump ID not found")?;

    // Trigger LLM analysis in background (don't fail if it errors)
    if let LLMProvider::Disabled = config.provider {
        // Skip analysis if LLM is disabled
        return Ok(mind_dump);
    }

    // Spawn async LLM analysis in background
    // Open a new database connection for the background task
    let content = input.content.clone();
    
    tokio::spawn(async move {
        // Open a new database connection for this background task
        let background_db = match Database::new() {
            Ok(db) => db,
            Err(e) => {
                eprintln!("Failed to open database for background analysis: {}", e);
                return;
            }
        };

        // Run analysis
        match client::generate_mind_dump_analysis(&content, &config).await {
            Ok(analysis_response) => {
                // Validate that mind dump still exists before creating analysis
                // This prevents foreign key constraint errors if user deleted the mind dump
                match background_db.get_mind_dump(mind_dump_id) {
                    Ok(Some(_)) => {
                        // Mind dump exists, proceed with analysis
                    }
                    Ok(None) => {
                        eprintln!("Mind dump {} was deleted before analysis could complete", mind_dump_id);
                        return;
                    }
                    Err(e) => {
                        eprintln!("Failed to verify mind dump {} exists: {}", mind_dump_id, e);
                        return;
                    }
                }

                // Serialize blocker patterns
                let blocker_patterns_json = if analysis_response.blocker_patterns.is_empty() {
                    None
                } else {
                    serde_json::to_string(&analysis_response.blocker_patterns).ok()
                };

                // Create analysis entry - handle foreign key constraint errors gracefully
                match background_db.create_mind_dump_analysis(mind_dump_id) {
                    Ok(analysis) => {
                        let analysis_id = match analysis.id {
                            Some(id) => id,
                            None => {
                                eprintln!("Failed to get analysis ID after creation");
                                return;
                            }
                        };

                        // Link cards to analysis
                        for symbol_card in analysis_response.relevant_cards {
                            // Get card by name
                            if let Ok(Some(card)) = background_db.get_card_by_name(&symbol_card.card_name) {
                                if let Some(card_id) = card.id {
                                    if let Err(e) = background_db.link_card_to_mind_dump_analysis(
                                        analysis_id,
                                        card_id,
                                        Some(symbol_card.relevance_note),
                                    ) {
                                        eprintln!("Failed to link card '{}' to analysis: {}", symbol_card.card_name, e);
                                    }
                                } else {
                                    eprintln!("Card '{}' has no ID", symbol_card.card_name);
                                }
                            } else {
                                eprintln!("Card '{}' not found in database", symbol_card.card_name);
                            }
                        }

                        // Store tasks
                        for task in analysis_response.tasks {
                            let task_title = task.title.clone();
                            if let Err(e) = background_db.create_mind_dump_analysis_task(
                                analysis_id,
                                task.title,
                                task.description,
                            ) {
                                eprintln!("Failed to create task '{}': {}", task_title, e);
                            }
                        }

                        // Store mood tags - verify mind dump still exists
                        if !analysis_response.mood_tags.is_empty() {
                            // Double-check mind dump exists before updating
                            if background_db.get_mind_dump(mind_dump_id).is_ok_and(|r| r.is_some()) {
                                let mood_tags_json = serde_json::to_string(&analysis_response.mood_tags)
                                    .unwrap_or_else(|_| "[]".to_string());
                                if let Err(e) = background_db.update_mind_dump_mood_tags(mind_dump_id, Some(mood_tags_json)) {
                                    eprintln!("Failed to update mood tags for mind_dump_id {}: {}", mind_dump_id, e);
                                }
                            } else {
                                eprintln!("Mind dump {} was deleted before mood tags could be updated", mind_dump_id);
                            }
                        }

                        // Store blocker patterns
                        if let Some(blocker_patterns_json) = &blocker_patterns_json {
                            if let Err(e) = background_db.update_mind_dump_analysis_blocker_patterns(analysis_id, Some(blocker_patterns_json.clone())) {
                                eprintln!("Failed to update blocker patterns: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        // Check if error is due to foreign key constraint (mind dump deleted)
                        let error_msg = e.to_string();
                        if error_msg.contains("FOREIGN KEY") || error_msg.contains("foreign key") {
                            eprintln!("Mind dump {} was deleted during analysis, skipping analysis creation", mind_dump_id);
                        } else {
                            eprintln!("Failed to create mind dump analysis for mind_dump_id {}: {}", mind_dump_id, e);
                        }
                        return;
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to analyze mind dump: {}", e);
            }
        }
    });

    Ok(mind_dump)
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

// Card commands (read-only - cards are a fixed deck like tarot)
#[tauri::command]
pub fn get_card(
    db: State<Database>,
    id: i64,
) -> Result<Option<Card>, String> {
    db.get_card(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_card_by_name(
    db: State<Database>,
    name: String,
) -> Result<Option<Card>, String> {
    db.get_card_by_name(&name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_cards(
    db: State<Database>,
) -> Result<Vec<Card>, String> {
    db.list_cards().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_cards_by_usage(
    db: State<Database>,
) -> Result<Vec<CardWithCount>, String> {
    db.list_cards_by_usage().map_err(|e| e.to_string())
}

// Bug-Card relationship commands
#[tauri::command]
pub fn create_bug_with_cards(
    db: State<Database>,
    input: CreateBugInput,
    card_names: Vec<String>,
) -> Result<Bug, String> {
    db.create_bug_with_cards(input, card_names).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn link_card_to_bug(
    db: State<Database>,
    bug_id: i64,
    card_id: i64,
    position: Option<i32>,
) -> Result<BugCard, String> {
    db.link_card_to_bug(bug_id, card_id, position).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_bug_cards(
    db: State<Database>,
    bug_id: i64,
) -> Result<Vec<Card>, String> {
    db.get_bug_cards(bug_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn unlink_card_from_bug(
    db: State<Database>,
    bug_id: i64,
    card_id: i64,
) -> Result<bool, String> {
    db.unlink_card_from_bug(bug_id, card_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn clear_bug_cards(
    db: State<Database>,
    bug_id: i64,
) -> Result<(), String> {
    db.clear_bug_cards(bug_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_card_bugs(
    db: State<Database>,
    card_id: i64,
) -> Result<Vec<Bug>, String> {
    db.get_card_bugs(card_id).map_err(|e| e.to_string())
}

// Database backup command
#[tauri::command]
pub fn backup_database(
    db: State<Database>,
    destination: String,
) -> Result<(), String> {
    let dest_path = PathBuf::from(destination);
    db.backup_database(&dest_path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_database_path() -> Result<String, String> {
    Database::get_database_path_public()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}

// Mind dump analysis
#[tauri::command]
pub async fn analyze_mind_dump(
    content: String,
    config: LLMConfig,
) -> Result<GenerateMindDumpAnalysisResponse, String> {
    // Call LLM to generate analysis
    client::generate_mind_dump_analysis(&content, &config).await
}

#[tauri::command]
pub fn get_mind_dump_analysis_with_cards_and_tasks(
    db: State<Database>,
    mind_dump_id: i64,
) -> Result<Option<MindDumpAnalysisWithCardsAndTasks>, String> {
    db.get_mind_dump_analysis_with_cards_and_tasks(mind_dump_id).map_err(|e| e.to_string())
}
