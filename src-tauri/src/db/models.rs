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
