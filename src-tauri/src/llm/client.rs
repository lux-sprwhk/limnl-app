use super::types::{LLMConfig, LLMProvider};
use reqwest;
use serde_json::{json, Value};

const TITLE_GENERATION_PROMPT: &str = r#"You are a helpful assistant that generates concise, evocative titles for dream journal entries.

Given the dream content below, generate a short title (2-6 words) that captures the essence of the dream. The title should be memorable and help the dreamer recall the dream later.

Rules:
- Keep it brief (2-6 words)
- Capture the main theme or memorable element
- Don't use quotes around the title
- Respond with ONLY the title, nothing else

Dream content:"#;

const DESCRIPTION_OPTIMIZATION_PROMPT: &str = r#"You are a helpful assistant that optimizes dream journal descriptions for clarity, analysis, and data mining.

Given the raw dream description below, improve it by:
- Organizing the narrative into a clear, coherent structure
- Clarifying ambiguous descriptions
- Highlighting key symbols, emotions, and themes
- Maintaining the dreamer's original voice and perspective
- Making it more suitable for analysis while preserving authenticity

Respond with ONLY the optimized description, nothing else.

Raw dream description:"#;

const CARD_COMMENTARY_PROMPT: &str = r#"You are a compassionate guide helping someone discover what's really bothering them through reflective card-based inquiry.

A person is exploring a bug or issue in their {life_area} area. They've drawn a card that might help them understand what's blocking them.

Card: {card_name}
Question: {card_question}
Meaning: {card_meaning}

Provide a brief, insightful commentary (1-2 short sentences) on how this card's meaning might relate to issues in their {life_area} area. Help them see potential connections without being prescriptive.

Keep the commentary concise and to the point.
Respond with ONLY the commentary, nothing else."#;

fn map_ollama_model(model_name: &str) -> &str {
    match model_name {
        "llama" => "llama3.2",
        "mistral" => "mistral",
        "phi" => "phi3",
        "deepseek" => "deepseek-coder",
        _ => model_name,
    }
}

fn map_openai_model(model_name: &str) -> &str {
    match model_name {
        "gpt4-mini" => "gpt-4o-mini",
        "gpt4-turbo" => "gpt-4-turbo",
        "gpt4" => "gpt-4",
        "gpt4o" => "gpt-4o",
        _ => model_name,
    }
}

fn map_anthropic_model(model_name: &str) -> &str {
    match model_name {
        "claude-haiku" => "claude-haiku-4-5",
        "claude-sonnet" => "claude-sonnet-4-5",
        _ => model_name,
    }
}

pub async fn generate_title(content: &str, config: &LLMConfig) -> Result<String, String> {
    match config.provider {
        LLMProvider::Disabled => Err("LLM is disabled".to_string()),
        LLMProvider::Ollama => generate_title_ollama(content, config).await,
        LLMProvider::OpenAI => generate_title_openai(content, config).await,
        LLMProvider::Anthropic => generate_title_anthropic(content, config).await,
    }
}

pub async fn optimize_description(content: &str, config: &LLMConfig) -> Result<String, String> {
    match config.provider {
        LLMProvider::Disabled => Err("LLM is disabled".to_string()),
        LLMProvider::Ollama => optimize_description_ollama(content, config).await,
        LLMProvider::OpenAI => optimize_description_openai(content, config).await,
        LLMProvider::Anthropic => optimize_description_anthropic(content, config).await,
    }
}

pub async fn comment_on_card(
    card_name: &str,
    card_question: &str,
    card_meaning: &str,
    life_area: &str,
    config: &LLMConfig,
) -> Result<String, String> {
    match config.provider {
        LLMProvider::Disabled => Err("LLM is disabled".to_string()),
        LLMProvider::Ollama => comment_on_card_ollama(card_name, card_question, card_meaning, life_area, config).await,
        LLMProvider::OpenAI => comment_on_card_openai(card_name, card_question, card_meaning, life_area, config).await,
        LLMProvider::Anthropic => comment_on_card_anthropic(card_name, card_question, card_meaning, life_area, config).await,
    }
}

async fn generate_title_ollama(content: &str, config: &LLMConfig) -> Result<String, String> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/generate", config.ollama_url);
    let model = map_ollama_model(&config.ollama_model);

    let prompt = format!("{}\n\n{}", TITLE_GENERATION_PROMPT, content);

    let response = client
        .post(&url)
        .json(&json!({
            "model": model,
            "prompt": prompt,
            "stream": false
        }))
        .send()
        .await
        .map_err(|e| format!("Ollama request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Ollama API error: {}", response.status()));
    }

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Ollama response: {}", e))?;

    data.get("response")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .ok_or_else(|| "Invalid Ollama response format".to_string())
}

async fn generate_title_openai(content: &str, config: &LLMConfig) -> Result<String, String> {
    let client = reqwest::Client::new();
    let model = map_openai_model(&config.openai_model);

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", config.openai_api_key))
        .json(&json!({
            "model": model,
            "messages": [
                {
                    "role": "system",
                    "content": TITLE_GENERATION_PROMPT
                },
                {
                    "role": "user",
                    "content": content
                }
            ],
            "temperature": 0.7,
            "max_tokens": 20
        }))
        .send()
        .await
        .map_err(|e| format!("OpenAI request failed: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("OpenAI API error: {}", error_text));
    }

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;

    data.get("choices")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|choice| choice.get("message"))
        .and_then(|msg| msg.get("content"))
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .ok_or_else(|| "Invalid OpenAI response format".to_string())
}

async fn generate_title_anthropic(content: &str, config: &LLMConfig) -> Result<String, String> {
    let client = reqwest::Client::new();
    let model = map_anthropic_model(&config.anthropic_model);

    let prompt = format!("{}\n\n{}", TITLE_GENERATION_PROMPT, content);

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("Content-Type", "application/json")
        .header("x-api-key", &config.anthropic_api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&json!({
            "model": model,
            "max_tokens": 20,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        }))
        .send()
        .await
        .map_err(|e| format!("Anthropic request failed: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Anthropic API error: {}", error_text));
    }

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Anthropic response: {}", e))?;

    data.get("content")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|item| item.get("text"))
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .ok_or_else(|| "Invalid Anthropic response format".to_string())
}

async fn optimize_description_ollama(content: &str, config: &LLMConfig) -> Result<String, String> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/generate", config.ollama_url);
    let model = map_ollama_model(&config.ollama_model);

    let prompt = format!("{}\n\n{}", DESCRIPTION_OPTIMIZATION_PROMPT, content);

    let response = client
        .post(&url)
        .json(&json!({
            "model": model,
            "prompt": prompt,
            "stream": false
        }))
        .send()
        .await
        .map_err(|e| format!("Ollama request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Ollama API error: {}", response.status()));
    }

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Ollama response: {}", e))?;

    data.get("response")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .ok_or_else(|| "Invalid Ollama response format".to_string())
}

async fn optimize_description_openai(content: &str, config: &LLMConfig) -> Result<String, String> {
    let client = reqwest::Client::new();
    let model = map_openai_model(&config.openai_model);

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", config.openai_api_key))
        .json(&json!({
            "model": model,
            "messages": [
                {
                    "role": "system",
                    "content": DESCRIPTION_OPTIMIZATION_PROMPT
                },
                {
                    "role": "user",
                    "content": content
                }
            ],
            "temperature": 0.7,
            "max_tokens": 2000
        }))
        .send()
        .await
        .map_err(|e| format!("OpenAI request failed: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("OpenAI API error: {}", error_text));
    }

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;

    data.get("choices")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|choice| choice.get("message"))
        .and_then(|msg| msg.get("content"))
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .ok_or_else(|| "Invalid OpenAI response format".to_string())
}

async fn optimize_description_anthropic(content: &str, config: &LLMConfig) -> Result<String, String> {
    let client = reqwest::Client::new();
    let model = map_anthropic_model(&config.anthropic_model);

    let prompt = format!("{}\n\n{}", DESCRIPTION_OPTIMIZATION_PROMPT, content);

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("Content-Type", "application/json")
        .header("x-api-key", &config.anthropic_api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&json!({
            "model": model,
            "max_tokens": 2000,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        }))
        .send()
        .await
        .map_err(|e| format!("Anthropic request failed: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Anthropic API error: {}", error_text));
    }

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Anthropic response: {}", e))?;

    data.get("content")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|item| item.get("text"))
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .ok_or_else(|| "Invalid Anthropic response format".to_string())
}

async fn comment_on_card_ollama(
    card_name: &str,
    card_question: &str,
    card_meaning: &str,
    life_area: &str,
    config: &LLMConfig,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/generate", config.ollama_url);
    let model = map_ollama_model(&config.ollama_model);

    let prompt = CARD_COMMENTARY_PROMPT
        .replace("{life_area}", life_area)
        .replace("{card_name}", card_name)
        .replace("{card_question}", card_question)
        .replace("{card_meaning}", card_meaning);

    let response = client
        .post(&url)
        .json(&json!({
            "model": model,
            "prompt": prompt,
            "stream": false
        }))
        .send()
        .await
        .map_err(|e| format!("Ollama request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Ollama API error: {}", response.status()));
    }

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Ollama response: {}", e))?;

    data.get("response")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .ok_or_else(|| "Invalid Ollama response format".to_string())
}

async fn comment_on_card_openai(
    card_name: &str,
    card_question: &str,
    card_meaning: &str,
    life_area: &str,
    config: &LLMConfig,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let model = map_openai_model(&config.openai_model);

    let prompt = CARD_COMMENTARY_PROMPT
        .replace("{life_area}", life_area)
        .replace("{card_name}", card_name)
        .replace("{card_question}", card_question)
        .replace("{card_meaning}", card_meaning);

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", config.openai_api_key))
        .json(&json!({
            "model": model,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": 0.7,
            "max_tokens": 200
        }))
        .send()
        .await
        .map_err(|e| format!("OpenAI request failed: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("OpenAI API error: {}", error_text));
    }

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;

    data.get("choices")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|choice| choice.get("message"))
        .and_then(|msg| msg.get("content"))
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .ok_or_else(|| "Invalid OpenAI response format".to_string())
}

async fn comment_on_card_anthropic(
    card_name: &str,
    card_question: &str,
    card_meaning: &str,
    life_area: &str,
    config: &LLMConfig,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let model = map_anthropic_model(&config.anthropic_model);

    let prompt = CARD_COMMENTARY_PROMPT
        .replace("{life_area}", life_area)
        .replace("{card_name}", card_name)
        .replace("{card_question}", card_question)
        .replace("{card_meaning}", card_meaning);

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("Content-Type", "application/json")
        .header("x-api-key", &config.anthropic_api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&json!({
            "model": model,
            "max_tokens": 200,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        }))
        .send()
        .await
        .map_err(|e| format!("Anthropic request failed: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Anthropic API error: {}", error_text));
    }

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Anthropic response: {}", e))?;

    data.get("content")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|item| item.get("text"))
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .ok_or_else(|| "Invalid Anthropic response format".to_string())
}
