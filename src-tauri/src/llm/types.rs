use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LLMProvider {
    Disabled,
    Ollama,
    OpenAI,
    Anthropic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub provider: LLMProvider,
    #[serde(rename = "ollamaUrl")]
    pub ollama_url: String,
    #[serde(rename = "ollamaModel")]
    pub ollama_model: String,
    #[serde(rename = "openaiApiKey")]
    pub openai_api_key: String,
    #[serde(rename = "openaiModel")]
    pub openai_model: String,
    #[serde(rename = "anthropicApiKey")]
    pub anthropic_api_key: String,
    #[serde(rename = "anthropicModel")]
    pub anthropic_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateTitleRequest {
    pub content: String,
    pub config: LLMConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateTitleResponse {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizeDescriptionRequest {
    pub content: String,
    pub config: LLMConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizeDescriptionResponse {
    pub optimized: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardCommentaryResponse {
    pub commentary: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateDreamAnalysisRequest {
    pub dream_id: i64,
    pub dream_title: String,
    pub dream_content: String,
    pub sleep_quality: Option<i32>,
    pub config: LLMConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolCard {
    pub card_name: String,
    pub relevance_note: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateDreamAnalysisResponse {
    pub themes_patterns: String,
    pub emotional_analysis: String,
    pub narrative_summary: String,
    pub symbol_cards: Vec<SymbolCard>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateCreativePromptsRequest {
    pub dream_analysis_id: i64,
    pub themes_patterns: String,
    pub emotional_analysis: String,
    pub narrative_summary: String,
    pub config: LLMConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateCreativePromptsResponse {
    pub image_prompts: Vec<String>,
    pub music_prompts: Vec<String>,
    pub story_prompts: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateMindDumpAnalysisRequest {
    pub mind_dump_id: i64,
    pub content: String,
    pub config: LLMConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MindDumpTask {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateMindDumpAnalysisResponse {
    pub relevant_cards: Vec<SymbolCard>,
    pub tasks: Vec<MindDumpTask>,
    pub mood_tags: Vec<String>,
    pub blocker_patterns: Vec<String>,
}
