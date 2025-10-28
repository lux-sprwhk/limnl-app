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
