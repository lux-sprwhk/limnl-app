use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dream {
    pub id: Option<i64>,
    pub date_recorded: DateTime<Utc>,
    pub date_occurred: DateTime<Utc>,
    pub title: String,
    pub content: String,
    pub emotions_tags: Option<String>, // JSON string of tags
    pub sleep_quality: Option<i32>,    // 1-5 scale
    pub is_recurring: Option<bool>,    // Whether this dream has occurred before
    pub last_occurrence_period: Option<String>, // When it last occurred (e.g., "last_week")
    pub is_lucid: Option<bool>,        // Whether dreamer was aware they were dreaming
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDreamInput {
    pub date_occurred: DateTime<Utc>,
    pub title: String,
    pub content: String,
    pub emotions_tags: Option<String>,
    pub sleep_quality: Option<i32>,
    pub is_recurring: Option<bool>,
    pub last_occurrence_period: Option<String>,
    pub is_lucid: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDreamInput {
    pub id: i64,
    pub date_occurred: Option<DateTime<Utc>>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub emotions_tags: Option<String>,
    pub sleep_quality: Option<i32>,
    pub is_recurring: Option<bool>,
    pub last_occurrence_period: Option<String>,
    pub is_lucid: Option<bool>,
}

// Dream analysis models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamAnalysis {
    pub id: Option<i64>,
    pub dream_id: i64,
    pub themes_patterns: String,
    pub emotional_analysis: String,
    pub narrative_summary: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDreamAnalysisInput {
    pub dream_id: i64,
    pub themes_patterns: String,
    pub emotional_analysis: String,
    pub narrative_summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamAnalysisCard {
    pub dream_analysis_id: i64,
    pub card_id: i64,
    pub card_name: String,
    pub relevance_note: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamAnalysisWithCards {
    pub analysis: DreamAnalysis,
    pub cards: Vec<DreamAnalysisCard>,
}

// Dream creative prompts models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamCreativePrompts {
    pub id: Option<i64>,
    pub dream_analysis_id: i64,
    pub image_prompts: String, // JSON array of image prompts
    pub music_prompts: String, // JSON array of music prompts
    pub story_prompts: String, // JSON array of story prompts
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDreamCreativePromptsInput {
    pub dream_analysis_id: i64,
    pub image_prompts: String,
    pub music_prompts: String,
    pub story_prompts: String,
}

// Bug tracking models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bug {
    pub id: Option<i64>,
    pub title: String,
    pub description: String,
    pub status: String, // "active", "resolved", "archived"
    pub cards_drawn: Option<String>, // JSON string of card IDs
    pub conversation_history: Option<String>, // JSON string of conversation
    pub notes: Option<String>, // JSON string of notes array
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBugInput {
    pub title: String,
    pub description: String,
    pub cards_drawn: Option<String>,
    pub conversation_history: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBugInput {
    pub id: i64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub cards_drawn: Option<String>,
    pub conversation_history: Option<String>,
    pub notes: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
}

// Mind dump models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindDump {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub content: String,
    pub character_count: i32,
    pub mood_tags: Option<String>, // JSON string of mood tags
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMindDumpInput {
    pub title: Option<String>,
    pub content: String,
    pub character_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMindDumpInput {
    pub id: i64,
    pub title: Option<String>,
    pub content: Option<String>,
    pub character_count: Option<i32>,
}

// Mind dump analysis models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindDumpAnalysis {
    pub id: Option<i64>,
    pub mind_dump_id: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindDumpAnalysisCard {
    pub mind_dump_analysis_id: i64,
    pub card_id: i64,
    pub card_name: String,
    pub relevance_note: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindDumpAnalysisTask {
    pub id: Option<i64>,
    pub mind_dump_analysis_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindDumpAnalysisWithCards {
    pub analysis: MindDumpAnalysis,
    pub cards: Vec<MindDumpAnalysisCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindDumpAnalysisWithCardsAndTasks {
    pub analysis: MindDumpAnalysis,
    pub cards: Vec<MindDumpAnalysisCard>,
    pub tasks: Vec<MindDumpAnalysisTask>,
}

// Card models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: Option<i64>,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCardInput {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BugCard {
    pub bug_id: i64,
    pub card_id: i64,
    pub position: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardWithCount {
    pub id: i64,
    pub name: String,
    pub bug_count: i32,
    pub created_at: DateTime<Utc>,
}
