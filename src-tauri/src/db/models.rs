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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDreamInput {
    pub id: i64,
    pub date_occurred: Option<DateTime<Utc>>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub emotions_tags: Option<String>,
    pub sleep_quality: Option<i32>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBugInput {
    pub id: i64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub cards_drawn: Option<String>,
    pub conversation_history: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
}

// Mind dump models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindDump {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub content: String,
    pub word_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMindDumpInput {
    pub title: Option<String>,
    pub content: String,
    pub word_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMindDumpInput {
    pub id: i64,
    pub title: Option<String>,
    pub content: Option<String>,
    pub word_count: Option<i32>,
}
