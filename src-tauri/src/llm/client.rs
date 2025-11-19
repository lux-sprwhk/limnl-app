use super::types::{LLMConfig, LLMProvider, GenerateDreamAnalysisResponse, GenerateCreativePromptsResponse, GenerateMindDumpAnalysisResponse};
use super::prompts::{
    TITLE_GENERATION_PROMPT, DESCRIPTION_OPTIMIZATION_PROMPT, CARD_COMMENTARY_PROMPT,
    CARD_COMMENTARY_WITH_CONTEXT_PROMPT, MULTIPLE_CARDS_COMMENTARY_PROMPT,
    get_discovery_chat_system_prompt, DREAM_ANALYSIS_PROMPT, CREATIVE_PROMPTS_GENERATION,
    get_mind_dump_analysis_prompt,
};
use super::helpers::{extract_card_summaries, extract_card_summaries_with_tags};
use reqwest;
use serde_json::{json, Value};

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

/// Safely extract JSON from markdown code blocks or plain text
/// Looks for ```json ... ``` blocks first, then falls back to finding { ... } in text
fn extract_json_from_response(response_text: &str) -> Result<String, String> {
    // First, try to find markdown code blocks with json language tag
    if let Some(start_marker) = response_text.find("```json") {
        let code_start = start_marker + 7; // Length of "```json"
        // Find the closing ```
        if let Some(end_marker) = response_text[code_start..].find("```") {
            let json_str = response_text[code_start..code_start + end_marker].trim();
            // Validate it looks like JSON (starts with { or [)
            if json_str.starts_with('{') || json_str.starts_with('[') {
                return Ok(json_str.to_string());
            }
        }
    }
    
    // Also try ``` without language tag
    if let Some(start_marker) = response_text.find("```") {
        let code_start = start_marker + 3; // Length of "```"
        if let Some(end_marker) = response_text[code_start..].find("```") {
            let json_str = response_text[code_start..code_start + end_marker].trim();
            // Validate it looks like JSON
            if (json_str.starts_with('{') || json_str.starts_with('[')) && json_str.len() > 2 {
                return Ok(json_str.to_string());
            }
        }
    }
    
    // Fall back to finding JSON object/array in text
    // Look for opening brace/bracket
    let (json_start, opening_char, closing_char) = if let Some(pos) = response_text.find('{') {
        (pos, '{', '}')
    } else if let Some(pos) = response_text.find('[') {
        (pos, '[', ']')
    } else {
        return Err("No JSON object or array found in response".to_string());
    };
    
    // Find matching closing brace/bracket by counting nested braces
    // Start with depth 1 since we're already at the opening character
    let mut depth = 1;
    let mut in_string = false;
    let mut escape_next = false;
    let mut json_end = None;
    
    // Iterate through characters, tracking byte positions
    let mut chars = response_text[json_start + 1..].char_indices();
    
    while let Some((char_byte_offset, ch)) = chars.next() {
        // char_byte_offset is relative to json_start + 1, so absolute position is:
        let absolute_pos = json_start + 1 + char_byte_offset;
        
        if escape_next {
            escape_next = false;
            continue;
        }
        
        match ch {
            '\\' if in_string => {
                escape_next = true;
            }
            '"' => {
                in_string = !in_string;
            }
            c if c == opening_char && !in_string => {
                depth += 1;
            }
            c if c == closing_char && !in_string => {
                depth -= 1;
                if depth == 0 {
                    // Include the closing character in the slice
                    json_end = Some(absolute_pos + ch.len_utf8() - 1);
                    break;
                }
            }
            _ => {}
        }
    }
    
    let json_end = json_end.ok_or("No matching closing brace/bracket found in JSON".to_string())?;
    let json_str = &response_text[json_start..=json_end];
    
    Ok(json_str.to_string())
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

pub async fn comment_on_multiple_cards(
    cards: &[Value],
    life_area: &str,
    config: &LLMConfig,
) -> Result<std::collections::HashMap<String, String>, String> {
    match config.provider {
        LLMProvider::Disabled => Err("LLM is disabled".to_string()),
        LLMProvider::Ollama => comment_on_multiple_cards_ollama(cards, life_area, config).await,
        LLMProvider::OpenAI => comment_on_multiple_cards_openai(cards, life_area, config).await,
        LLMProvider::Anthropic => comment_on_multiple_cards_anthropic(cards, life_area, config).await,
    }
}

pub async fn comment_on_card_with_context(
    card_name: &str,
    card_question: &str,
    card_meaning: &str,
    life_area: &str,
    selected_cards: &[Value],
    config: &LLMConfig,
) -> Result<String, String> {
    match config.provider {
        LLMProvider::Disabled => Err("LLM is disabled".to_string()),
        LLMProvider::Ollama => comment_on_card_with_context_ollama(card_name, card_question, card_meaning, life_area, selected_cards, config).await,
        LLMProvider::OpenAI => comment_on_card_with_context_openai(card_name, card_question, card_meaning, life_area, selected_cards, config).await,
        LLMProvider::Anthropic => comment_on_card_with_context_anthropic(card_name, card_question, card_meaning, life_area, selected_cards, config).await,
    }
}

pub async fn comment_on_multiple_cards_with_context(
    cards: &[Value],
    life_area: &str,
    selected_cards: &[Value],
    config: &LLMConfig,
) -> Result<std::collections::HashMap<String, String>, String> {
    match config.provider {
        LLMProvider::Disabled => Err("LLM is disabled".to_string()),
        LLMProvider::Ollama => comment_on_multiple_cards_with_context_ollama(cards, life_area, selected_cards, config).await,
        LLMProvider::OpenAI => comment_on_multiple_cards_with_context_openai(cards, life_area, selected_cards, config).await,
        LLMProvider::Anthropic => comment_on_multiple_cards_with_context_anthropic(cards, life_area, selected_cards, config).await,
    }
}

pub async fn chat_with_history_with_profile(
    user_message: &str,
    messages: &[Value],
    card_name: &str,
    card_question: &str,
    card_meaning: &str,
    card_insights: &str,
    life_area: &str,
    user_name: &str,
    zodiac_sign: Option<&str>,
    mbti_type: Option<&str>,
    selected_cards: &[Value],
    config: &LLMConfig,
) -> Result<String, String> {
    match config.provider {
        LLMProvider::Disabled => Err("LLM is disabled".to_string()),
        LLMProvider::Ollama => chat_with_history_ollama(user_message, messages, card_name, card_question, card_meaning, card_insights, life_area, user_name, zodiac_sign, mbti_type, selected_cards, config).await,
        LLMProvider::OpenAI => chat_with_history_openai(user_message, messages, card_name, card_question, card_meaning, card_insights, life_area, user_name, zodiac_sign, mbti_type, selected_cards, config).await,
        LLMProvider::Anthropic => chat_with_history_anthropic(user_message, messages, card_name, card_question, card_meaning, card_insights, life_area, user_name, zodiac_sign, mbti_type, selected_cards, config).await,
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

// Private implementation functions - these are called through match statements
// in public functions, so the compiler may not detect their usage. They are
// definitely used, so we allow dead_code warnings.
#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

async fn comment_on_multiple_cards_ollama(
    cards: &[Value],
    life_area: &str,
    config: &LLMConfig,
) -> Result<std::collections::HashMap<String, String>, String> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/generate", config.ollama_url);
    let model = map_ollama_model(&config.ollama_model);

    let mut cards_list = String::new();
    for card in cards {
        let id = card.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
        let name = card.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let question = card.get("question").and_then(|v| v.as_str()).unwrap_or("");
        let meaning = card.get("meaning").and_then(|v| v.as_str()).unwrap_or("");
        cards_list.push_str(&format!(
            "\nCard {}: {}\nQuestion: {}\nMeaning: {}\n",
            id, name, question, meaning
        ));
    }

    let prompt = MULTIPLE_CARDS_COMMENTARY_PROMPT
        .replace("{life_area}", life_area)
        .replace("{cards_list}", &cards_list);

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
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Ollama API error: {}", error_text));
    }

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Ollama response: {}", e))?;

    let response_text = data
        .get("response")
        .and_then(|v| v.as_str())
        .ok_or("Invalid Ollama response format")?;

    // Extract JSON from response (in case there's extra text)
    let json_str = extract_json_from_response(response_text)?;
    serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse JSON response: {}", e))
}

async fn comment_on_multiple_cards_openai(
    cards: &[Value],
    life_area: &str,
    config: &LLMConfig,
) -> Result<std::collections::HashMap<String, String>, String> {
    let client = reqwest::Client::new();
    let model = map_openai_model(&config.openai_model);

    let mut cards_list = String::new();
    for card in cards {
        let id = card.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
        let name = card.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let question = card.get("question").and_then(|v| v.as_str()).unwrap_or("");
        let meaning = card.get("meaning").and_then(|v| v.as_str()).unwrap_or("");
        cards_list.push_str(&format!(
            "\nCard {}: {}\nQuestion: {}\nMeaning: {}\n",
            id, name, question, meaning
        ));
    }

    let prompt = MULTIPLE_CARDS_COMMENTARY_PROMPT
        .replace("{life_area}", life_area)
        .replace("{cards_list}", &cards_list);

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
            "max_tokens": 500
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

    let response_text = data
        .get("choices")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|item| item.get("message"))
        .and_then(|msg| msg.get("content"))
        .and_then(|v| v.as_str())
        .ok_or("Invalid OpenAI response format")?;

    // Extract JSON from response (in case there's extra text)
    let json_str = extract_json_from_response(response_text)?;
    serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse JSON response: {}", e))
}

async fn comment_on_multiple_cards_anthropic(
    cards: &[Value],
    life_area: &str,
    config: &LLMConfig,
) -> Result<std::collections::HashMap<String, String>, String> {
    let client = reqwest::Client::new();
    let model = map_anthropic_model(&config.anthropic_model);

    let mut cards_list = String::new();
    for card in cards {
        let id = card.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
        let name = card.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let question = card.get("question").and_then(|v| v.as_str()).unwrap_or("");
        let meaning = card.get("meaning").and_then(|v| v.as_str()).unwrap_or("");
        cards_list.push_str(&format!(
            "\nCard {}: {}\nQuestion: {}\nMeaning: {}\n",
            id, name, question, meaning
        ));
    }

    let prompt = MULTIPLE_CARDS_COMMENTARY_PROMPT
        .replace("{life_area}", life_area)
        .replace("{cards_list}", &cards_list);

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("Content-Type", "application/json")
        .header("x-api-key", &config.anthropic_api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&json!({
            "model": model,
            "max_tokens": 1000,
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

    let response_text = data
        .get("content")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|item| item.get("text"))
        .and_then(|v| v.as_str())
        .ok_or("Invalid Anthropic response format")?;

    // Extract JSON from response (in case there's extra text)
    let json_str = extract_json_from_response(response_text)?;
    serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse JSON response: {}", e))
}

// Chat with history implementations
async fn chat_with_history_ollama(
    user_message: &str,
    messages: &[Value],
    card_name: &str,
    card_question: &str,
    card_meaning: &str,
    card_insights: &str,
    life_area: &str,
    user_name: &str,
    zodiac_sign: Option<&str>,
    mbti_type: Option<&str>,
    selected_cards: &[Value],
    config: &LLMConfig,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/generate", config.ollama_url);
    let model = map_ollama_model(&config.ollama_model);

    // Build conversation context from message history
    let mut conversation = String::new();
    for msg in messages {
        if let (Some(role), Some(content)) = (msg.get("role").and_then(|v| v.as_str()), msg.get("content").and_then(|v| v.as_str())) {
            let role_label = if role == "user" { "User" } else { "Assistant" };
            conversation.push_str(&format!("{}: {}\n", role_label, content));
        }
    }

    // Build selected cards context
    let mut selected_cards_context = String::new();
    for card in selected_cards {
        if let (Some(name), Some(question), Some(commentary)) = (
            card.get("name").and_then(|v| v.as_str()),
            card.get("card_question").and_then(|v| v.as_str()),
            card.get("commentary").and_then(|v| v.as_str())
        ) {
            selected_cards_context.push_str(&format!(
                "- {}: {}\n  Commentary: {}\n",
                name, question, commentary
            ));
        }
    }

    let mut system_prompt = get_discovery_chat_system_prompt()
        .replace("{life_area}", life_area)
        .replace("{card_name}", card_name)
        .replace("{card_question}", card_question)
        .replace("{card_meaning}", card_meaning)
        .replace("{selected_cards_context}", if selected_cards_context.is_empty() { "No other cards selected yet." } else { &selected_cards_context });
    
    if !card_insights.is_empty() {
        system_prompt.push_str(&format!("\n\nCard Insights (generated for this life area):\n{}", card_insights));
    }
    
    // Add user profile context
    if !user_name.is_empty() || zodiac_sign.is_some() || mbti_type.is_some() {
        system_prompt.push_str("\n\nUser Profile:");
        if !user_name.is_empty() {
            system_prompt.push_str(&format!("\n- Name: {}", user_name));
        }
        if let Some(sign) = zodiac_sign {
            system_prompt.push_str(&format!("\n- Zodiac Sign: {}", sign));
        }
        if let Some(mbti) = mbti_type {
            system_prompt.push_str(&format!("\n- MBTI Type: {}", mbti));
        }
    }

    let prompt = format!("{}\n\nConversation history:\n{}\nUser: {}\n\nAssistant:", system_prompt, conversation, user_message);

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

async fn chat_with_history_openai(
    user_message: &str,
    messages: &[Value],
    card_name: &str,
    card_question: &str,
    card_meaning: &str,
    card_insights: &str,
    life_area: &str,
    user_name: &str,
    zodiac_sign: Option<&str>,
    mbti_type: Option<&str>,
    selected_cards: &[Value],
    config: &LLMConfig,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let model = map_openai_model(&config.openai_model);

    // Build selected cards context
    let mut selected_cards_context = String::new();
    for card in selected_cards {
        if let (Some(name), Some(question), Some(commentary)) = (
            card.get("name").and_then(|v| v.as_str()),
            card.get("card_question").and_then(|v| v.as_str()),
            card.get("commentary").and_then(|v| v.as_str())
        ) {
            selected_cards_context.push_str(&format!(
                "- {}: {}\n  Commentary: {}\n",
                name, question, commentary
            ));
        }
    }

    let mut system_prompt = get_discovery_chat_system_prompt()
        .replace("{life_area}", life_area)
        .replace("{card_name}", card_name)
        .replace("{card_question}", card_question)
        .replace("{card_meaning}", card_meaning)
        .replace("{selected_cards_context}", if selected_cards_context.is_empty() { "No other cards selected yet." } else { &selected_cards_context });
    
    if !card_insights.is_empty() {
        system_prompt.push_str(&format!("\n\nCard Insights (generated for this life area):\n{}", card_insights));
    }
    
    // Add user profile context
    if !user_name.is_empty() || zodiac_sign.is_some() || mbti_type.is_some() {
        system_prompt.push_str("\n\nUser Profile:");
        if !user_name.is_empty() {
            system_prompt.push_str(&format!("\n- Name: {}", user_name));
        }
        if let Some(sign) = zodiac_sign {
            system_prompt.push_str(&format!("\n- Zodiac Sign: {}", sign));
        }
        if let Some(mbti) = mbti_type {
            system_prompt.push_str(&format!("\n- MBTI Type: {}", mbti));
        }
    }

    // Build message array for OpenAI
    let mut chat_messages = vec![
        json!({
            "role": "system",
            "content": system_prompt
        })
    ];

    // Add conversation history
    for msg in messages {
        if let (Some(role), Some(content)) = (msg.get("role").and_then(|v| v.as_str()), msg.get("content").and_then(|v| v.as_str())) {
            chat_messages.push(json!({
                "role": role,
                "content": content
            }));
        }
    }

    // Add current user message
    chat_messages.push(json!({
        "role": "user",
        "content": user_message
    }));

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", config.openai_api_key))
        .json(&json!({
            "model": model,
            "messages": chat_messages,
            "temperature": 0.7,
            "max_tokens": 300
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

async fn chat_with_history_anthropic(
    user_message: &str,
    messages: &[Value],
    card_name: &str,
    card_question: &str,
    card_meaning: &str,
    card_insights: &str,
    life_area: &str,
    user_name: &str,
    zodiac_sign: Option<&str>,
    mbti_type: Option<&str>,
    selected_cards: &[Value],
    config: &LLMConfig,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let model = map_anthropic_model(&config.anthropic_model);

    // Build selected cards context
    let mut selected_cards_context = String::new();
    for card in selected_cards {
        if let (Some(name), Some(question), Some(commentary)) = (
            card.get("name").and_then(|v| v.as_str()),
            card.get("card_question").and_then(|v| v.as_str()),
            card.get("commentary").and_then(|v| v.as_str())
        ) {
            selected_cards_context.push_str(&format!(
                "- {}: {}\n  Commentary: {}\n",
                name, question, commentary
            ));
        }
    }

    let mut system_prompt = get_discovery_chat_system_prompt()
        .replace("{life_area}", life_area)
        .replace("{card_name}", card_name)
        .replace("{card_question}", card_question)
        .replace("{card_meaning}", card_meaning)
        .replace("{selected_cards_context}", if selected_cards_context.is_empty() { "No other cards selected yet." } else { &selected_cards_context });
    
    if !card_insights.is_empty() {
        system_prompt.push_str(&format!("\n\nCard Insights (generated for this life area):\n{}", card_insights));
    }
    
    // Add user profile context
    if !user_name.is_empty() || zodiac_sign.is_some() || mbti_type.is_some() {
        system_prompt.push_str("\n\nUser Profile:");
        if !user_name.is_empty() {
            system_prompt.push_str(&format!("\n- Name: {}", user_name));
        }
        if let Some(sign) = zodiac_sign {
            system_prompt.push_str(&format!("\n- Zodiac Sign: {}", sign));
        }
        if let Some(mbti) = mbti_type {
            system_prompt.push_str(&format!("\n- MBTI Type: {}", mbti));
        }
    }

    // Build message array for Anthropic
    let mut chat_messages = vec![];

    // Add conversation history
    for msg in messages {
        if let (Some(role), Some(content)) = (msg.get("role").and_then(|v| v.as_str()), msg.get("content").and_then(|v| v.as_str())) {
            chat_messages.push(json!({
                "role": role,
                "content": content
            }));
        }
    }

    // Add current user message
    chat_messages.push(json!({
        "role": "user",
        "content": user_message
    }));

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("Content-Type", "application/json")
        .header("x-api-key", &config.anthropic_api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&json!({
            "model": model,
            "max_tokens": 300,
            "system": system_prompt,
            "messages": chat_messages
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

// Context-aware card commentary functions
async fn comment_on_card_with_context_ollama(
    card_name: &str,
    card_question: &str,
    card_meaning: &str,
    life_area: &str,
    selected_cards: &[Value],
    config: &LLMConfig,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/generate", config.ollama_url);
    let model = map_ollama_model(&config.ollama_model);

    let mut selected_cards_list = String::new();
    for card in selected_cards {
        let name = card.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let question = card.get("question").and_then(|v| v.as_str()).unwrap_or("");
        let meaning = card.get("meaning").and_then(|v| v.as_str()).unwrap_or("");
        selected_cards_list.push_str(&format!(
            "- {}\n  Question: {}\n  Meaning: {}\n",
            name, question, meaning
        ));
    }

    let prompt = if selected_cards.is_empty() {
        CARD_COMMENTARY_PROMPT
            .replace("{life_area}", life_area)
            .replace("{card_name}", card_name)
            .replace("{card_question}", card_question)
            .replace("{card_meaning}", card_meaning)
    } else {
        CARD_COMMENTARY_WITH_CONTEXT_PROMPT
            .replace("{life_area}", life_area)
            .replace("{card_name}", card_name)
            .replace("{card_question}", card_question)
            .replace("{card_meaning}", card_meaning)
            .replace("{selected_cards_list}", &selected_cards_list)
    };

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

async fn comment_on_card_with_context_openai(
    card_name: &str,
    card_question: &str,
    card_meaning: &str,
    life_area: &str,
    selected_cards: &[Value],
    config: &LLMConfig,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let model = map_openai_model(&config.openai_model);

    let mut selected_cards_list = String::new();
    for card in selected_cards {
        let name = card.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let question = card.get("question").and_then(|v| v.as_str()).unwrap_or("");
        let meaning = card.get("meaning").and_then(|v| v.as_str()).unwrap_or("");
        selected_cards_list.push_str(&format!(
            "- {}\n  Question: {}\n  Meaning: {}\n",
            name, question, meaning
        ));
    }

    let prompt = if selected_cards.is_empty() {
        CARD_COMMENTARY_PROMPT
            .replace("{life_area}", life_area)
            .replace("{card_name}", card_name)
            .replace("{card_question}", card_question)
            .replace("{card_meaning}", card_meaning)
    } else {
        CARD_COMMENTARY_WITH_CONTEXT_PROMPT
            .replace("{life_area}", life_area)
            .replace("{card_name}", card_name)
            .replace("{card_question}", card_question)
            .replace("{card_meaning}", card_meaning)
            .replace("{selected_cards_list}", &selected_cards_list)
    };

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

async fn comment_on_card_with_context_anthropic(
    card_name: &str,
    card_question: &str,
    card_meaning: &str,
    life_area: &str,
    selected_cards: &[Value],
    config: &LLMConfig,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let model = map_anthropic_model(&config.anthropic_model);

    let mut selected_cards_list = String::new();
    for card in selected_cards {
        let name = card.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let question = card.get("question").and_then(|v| v.as_str()).unwrap_or("");
        let meaning = card.get("meaning").and_then(|v| v.as_str()).unwrap_or("");
        selected_cards_list.push_str(&format!(
            "- {}\n  Question: {}\n  Meaning: {}\n",
            name, question, meaning
        ));
    }

    let prompt = if selected_cards.is_empty() {
        CARD_COMMENTARY_PROMPT
            .replace("{life_area}", life_area)
            .replace("{card_name}", card_name)
            .replace("{card_question}", card_question)
            .replace("{card_meaning}", card_meaning)
    } else {
        CARD_COMMENTARY_WITH_CONTEXT_PROMPT
            .replace("{life_area}", life_area)
            .replace("{card_name}", card_name)
            .replace("{card_question}", card_question)
            .replace("{card_meaning}", card_meaning)
            .replace("{selected_cards_list}", &selected_cards_list)
    };

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

// Multiple cards with context implementations
async fn comment_on_multiple_cards_with_context_ollama(
    cards: &[Value],
    life_area: &str,
    _selected_cards: &[Value],
    config: &LLMConfig,
) -> Result<std::collections::HashMap<String, String>, String> {
    comment_on_multiple_cards_ollama(cards, life_area, config).await
}

async fn comment_on_multiple_cards_with_context_openai(
    cards: &[Value],
    life_area: &str,
    _selected_cards: &[Value],
    config: &LLMConfig,
) -> Result<std::collections::HashMap<String, String>, String> {
    comment_on_multiple_cards_openai(cards, life_area, config).await
}

async fn comment_on_multiple_cards_with_context_anthropic(
    cards: &[Value],
    life_area: &str,
    _selected_cards: &[Value],
    config: &LLMConfig,
) -> Result<std::collections::HashMap<String, String>, String> {
    comment_on_multiple_cards_anthropic(cards, life_area, config).await
}


// Dream analysis generation
pub async fn generate_dream_analysis(
    dream_title: &str,
    dream_content: &str,
    sleep_quality: Option<i32>,
    config: &LLMConfig,
) -> Result<GenerateDreamAnalysisResponse, String> {
    match config.provider {
        LLMProvider::Disabled => Err("LLM is disabled".to_string()),
        LLMProvider::Ollama => generate_dream_analysis_ollama(dream_title, dream_content, sleep_quality, config).await,
        LLMProvider::OpenAI => generate_dream_analysis_openai(dream_title, dream_content, sleep_quality, config).await,
        LLMProvider::Anthropic => generate_dream_analysis_anthropic(dream_title, dream_content, sleep_quality, config).await,
    }
}

async fn generate_dream_analysis_ollama(
    dream_title: &str,
    dream_content: &str,
    sleep_quality: Option<i32>,
    config: &LLMConfig,
) -> Result<GenerateDreamAnalysisResponse, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;
    let url = format!("{}/api/generate", config.ollama_url);
    let model = map_ollama_model(&config.ollama_model);

    // Extract simplified card summaries
    let card_summaries = extract_card_summaries()?;

    // Build the prompt with cards and dream info
    let prompt = DREAM_ANALYSIS_PROMPT.replace("{CARDS_JSON}", &card_summaries);
    let sleep_quality_text = match sleep_quality {
        Some(q) => format!("Sleep Quality: {}/5", q),
        None => "Sleep Quality: Not specified".to_string(),
    };

    let full_prompt = format!(
        "{}\n\nTitle: {}\n{}\n\nContent:\n{}",
        prompt, dream_title, sleep_quality_text, dream_content
    );

    eprintln!("Sending request to Ollama...");
    eprintln!("Model: {}", model);
    eprintln!("Prompt length: {} chars", full_prompt.len());

    let response = client
        .post(&url)
        .json(&json!({
            "model": model,
            "prompt": full_prompt,
            "stream": false
        }))
        .send()
        .await
        .map_err(|e| format!("Ollama request failed: {}", e))?;

    eprintln!("Received response with status: {}", response.status());

    if !response.status().is_success() {
        return Err(format!("Ollama API error: {}", response.status()));
    }

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Ollama response: {}", e))?;

    let response_text = data
        .get("response")
        .and_then(|v| v.as_str())
        .ok_or("Invalid Ollama response format")?;

    // Extract JSON from response
    let json_str = extract_json_from_response(response_text)?;
    serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse JSON response: {}", e))
}

async fn generate_dream_analysis_openai(
    dream_title: &str,
    dream_content: &str,
    sleep_quality: Option<i32>,
    config: &LLMConfig,
) -> Result<GenerateDreamAnalysisResponse, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;
    let model = map_openai_model(&config.openai_model);

    // Extract simplified card summaries
    let card_summaries = extract_card_summaries()?;

    // Build the prompt with cards
    let prompt = DREAM_ANALYSIS_PROMPT.replace("{CARDS_JSON}", &card_summaries);
    let sleep_quality_text = match sleep_quality {
        Some(q) => format!("Sleep Quality: {}/5", q),
        None => "Sleep Quality: Not specified".to_string(),
    };

    let user_message = format!(
        "Title: {}\n{}\n\nContent:\n{}",
        dream_title, sleep_quality_text, dream_content
    );

    eprintln!("Sending request to OpenAI API...");
    eprintln!("Model: {}", model);
    eprintln!("System prompt length: {} chars", prompt.len());
    eprintln!("User message length: {} chars", user_message.len());

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", config.openai_api_key))
        .json(&json!({
            "model": model,
            "messages": [
                {
                    "role": "system",
                    "content": prompt
                },
                {
                    "role": "user",
                    "content": user_message
                }
            ],
            "temperature": 0.7,
            "max_tokens": 1500
        }))
        .send()
        .await
        .map_err(|e| format!("OpenAI request failed: {}", e))?;

    eprintln!("Received response with status: {}", response.status());

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("OpenAI API error: {}", error_text));
    }

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;

    let response_text = data
        .get("choices")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|choice| choice.get("message"))
        .and_then(|msg| msg.get("content"))
        .and_then(|v| v.as_str())
        .ok_or("Invalid OpenAI response format")?;

    // Extract JSON from response
    let json_str = extract_json_from_response(response_text)?;
    serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse JSON response: {}", e))
}

async fn generate_dream_analysis_anthropic(
    dream_title: &str,
    dream_content: &str,
    sleep_quality: Option<i32>,
    config: &LLMConfig,
) -> Result<GenerateDreamAnalysisResponse, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;
    let model = map_anthropic_model(&config.anthropic_model);

    // Extract simplified card summaries
    let card_summaries = extract_card_summaries()?;

    // Build the prompt with cards
    let prompt = DREAM_ANALYSIS_PROMPT.replace("{CARDS_JSON}", &card_summaries);
    let sleep_quality_text = match sleep_quality {
        Some(q) => format!("Sleep Quality: {}/5", q),
        None => "Sleep Quality: Not specified".to_string(),
    };

    let user_message = format!(
        "{}\n\nTitle: {}\n{}\n\nContent:\n{}",
        prompt, dream_title, sleep_quality_text, dream_content
    );

    eprintln!("Sending request to Anthropic API...");
    eprintln!("Model: {}", model);
    eprintln!("Message length: {} chars", user_message.len());

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("Content-Type", "application/json")
        .header("x-api-key", &config.anthropic_api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&json!({
            "model": model,
            "max_tokens": 1500,
            "messages": [
                {
                    "role": "user",
                    "content": user_message
                }
            ]
        }))
        .send()
        .await
        .map_err(|e| format!("Anthropic request failed: {}", e))?;

    eprintln!("Received response with status: {}", response.status());

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Anthropic API error: {}", error_text));
    }

    eprintln!("Reading response body...");
    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Anthropic response: {}", e))?;
    eprintln!("Response body parsed successfully");

    eprintln!("Extracting text content from response...");
    let response_text = data
        .get("content")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|item| item.get("text"))
        .and_then(|v| v.as_str())
        .ok_or("Invalid Anthropic response format")?;
    eprintln!("Response text length: {} chars", response_text.len());

    // Extract JSON from response
    eprintln!("Extracting JSON from response text...");
    let json_str = extract_json_from_response(response_text)?;
    eprintln!("JSON string length: {} chars", json_str.len());

    eprintln!("Parsing JSON analysis response...");
    let result = serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse JSON response: {}", e))?;
    eprintln!("Analysis parsed successfully!");
    Ok(result)
}

// Creative Prompts Generation
pub async fn generate_creative_prompts(
    themes_patterns: &str,
    emotional_analysis: &str,
    narrative_summary: &str,
    config: &LLMConfig,
) -> Result<GenerateCreativePromptsResponse, String> {
    match config.provider {
        LLMProvider::Disabled => Err("LLM is disabled".to_string()),
        LLMProvider::Ollama => generate_creative_prompts_ollama(themes_patterns, emotional_analysis, narrative_summary, config).await,
        LLMProvider::OpenAI => generate_creative_prompts_openai(themes_patterns, emotional_analysis, narrative_summary, config).await,
        LLMProvider::Anthropic => generate_creative_prompts_anthropic(themes_patterns, emotional_analysis, narrative_summary, config).await,
    }
}

async fn generate_creative_prompts_ollama(
    themes_patterns: &str,
    emotional_analysis: &str,
    narrative_summary: &str,
    config: &LLMConfig,
) -> Result<GenerateCreativePromptsResponse, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;
    let url = format!("{}/api/generate", config.ollama_url);
    let model = map_ollama_model(&config.ollama_model);

    let analysis_summary = format!(
        "Themes & Patterns:\n{}\n\nEmotional Analysis:\n{}\n\nNarrative Summary:\n{}",
        themes_patterns, emotional_analysis, narrative_summary
    );

    let full_prompt = format!("{}\n\n{}", CREATIVE_PROMPTS_GENERATION, analysis_summary);

    let response = client
        .post(&url)
        .json(&json!({
            "model": model,
            "prompt": full_prompt,
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

    let response_text = data
        .get("response")
        .and_then(|v| v.as_str())
        .ok_or("Invalid Ollama response format")?;

    // Extract JSON from response
    let json_str = extract_json_from_response(response_text)?;
    serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse JSON response: {}", e))
}

async fn generate_creative_prompts_openai(
    themes_patterns: &str,
    emotional_analysis: &str,
    narrative_summary: &str,
    config: &LLMConfig,
) -> Result<GenerateCreativePromptsResponse, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;
    let model = map_openai_model(&config.openai_model);

    let analysis_summary = format!(
        "Themes & Patterns:\n{}\n\nEmotional Analysis:\n{}\n\nNarrative Summary:\n{}",
        themes_patterns, emotional_analysis, narrative_summary
    );

    let user_message = format!("{}\n\n{}", CREATIVE_PROMPTS_GENERATION, analysis_summary);

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", config.openai_api_key))
        .json(&json!({
            "model": model,
            "messages": [
                {"role": "user", "content": user_message}
            ],
            "response_format": { "type": "json_object" }
        }))
        .send()
        .await
        .map_err(|e| format!("OpenAI request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("OpenAI API error: {}", response.status()));
    }

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;

    let response_text = data
        .get("choices")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("message"))
        .and_then(|m| m.get("content"))
        .and_then(|c| c.as_str())
        .ok_or("Invalid OpenAI response format")?;

    serde_json::from_str(response_text)
        .map_err(|e| format!("Failed to parse JSON response: {}", e))
}

async fn generate_creative_prompts_anthropic(
    themes_patterns: &str,
    emotional_analysis: &str,
    narrative_summary: &str,
    config: &LLMConfig,
) -> Result<GenerateCreativePromptsResponse, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;
    let model = map_anthropic_model(&config.anthropic_model);

    let analysis_summary = format!(
        "Themes & Patterns:\n{}\n\nEmotional Analysis:\n{}\n\nNarrative Summary:\n{}",
        themes_patterns, emotional_analysis, narrative_summary
    );

    let user_message = format!("{}\n\n{}", CREATIVE_PROMPTS_GENERATION, analysis_summary);

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &config.anthropic_api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&json!({
            "model": model,
            "max_tokens": 2000,
            "messages": [
                {"role": "user", "content": user_message}
            ]
        }))
        .send()
        .await
        .map_err(|e| format!("Anthropic request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Anthropic API error: {}", response.status()));
    }

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Anthropic response: {}", e))?;

    let response_text = data
        .get("content")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("text"))
        .and_then(|t| t.as_str())
        .ok_or("Invalid Anthropic response format")?;

    // Extract JSON from response
    let json_str = extract_json_from_response(response_text)?;
    serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse JSON response: {}", e))
}

// Mind dump analysis generation
pub async fn generate_mind_dump_analysis(
    mind_dump_content: &str,
    config: &LLMConfig,
) -> Result<GenerateMindDumpAnalysisResponse, String> {
    match config.provider {
        LLMProvider::Disabled => Err("LLM is disabled".to_string()),
        LLMProvider::Ollama => generate_mind_dump_analysis_ollama(mind_dump_content, config).await,
        LLMProvider::OpenAI => generate_mind_dump_analysis_openai(mind_dump_content, config).await,
        LLMProvider::Anthropic => generate_mind_dump_analysis_anthropic(mind_dump_content, config).await,
    }
}

async fn generate_mind_dump_analysis_ollama(
    mind_dump_content: &str,
    config: &LLMConfig,
) -> Result<GenerateMindDumpAnalysisResponse, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;
    let url = format!("{}/api/generate", config.ollama_url);
    let model = map_ollama_model(&config.ollama_model);

    let card_summaries = extract_card_summaries_with_tags()?;
    let prompt = get_mind_dump_analysis_prompt().replace("{CARDS_SIMPLIFIED}", &card_summaries);
    let full_prompt = format!("{}\n\n{}", prompt, mind_dump_content);

    let response = client
        .post(&url)
        .json(&json!({
            "model": model,
            "prompt": full_prompt,
            "stream": false
        }))
        .send()
        .await
        .map_err(|e| format!("Ollama request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Ollama API error: {}", response.status()));
    }

    let data: Value = response.json().await.map_err(|e| format!("Failed to parse Ollama response: {}", e))?;
    let response_text = data.get("response").and_then(|v| v.as_str()).ok_or("Invalid Ollama response format")?;

    let json_str = extract_json_from_response(response_text)?;
    serde_json::from_str(&json_str).map_err(|e| format!("Failed to parse JSON response: {}", e))
}

async fn generate_mind_dump_analysis_openai(
    mind_dump_content: &str,
    config: &LLMConfig,
) -> Result<GenerateMindDumpAnalysisResponse, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;
    let model = map_openai_model(&config.openai_model);

    let card_summaries = extract_card_summaries_with_tags()?;
    let prompt = get_mind_dump_analysis_prompt().replace("{CARDS_SIMPLIFIED}", &card_summaries);
    let full_prompt = format!("{}\n\n{}", prompt, mind_dump_content);

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", config.openai_api_key))
        .json(&json!({
            "model": model,
            "messages": [{ "role": "user", "content": full_prompt }],
            "temperature": 0.7
        }))
        .send()
        .await
        .map_err(|e| format!("OpenAI request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("OpenAI API error: {}", response.status()));
    }

    let data: Value = response.json().await.map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;
    let response_text = data.get("choices").and_then(|v| v.as_array()).and_then(|arr| arr.get(0))
        .and_then(|choice| choice.get("message")).and_then(|msg| msg.get("content"))
        .and_then(|v| v.as_str()).ok_or("Invalid OpenAI response format")?;

    let json_str = extract_json_from_response(response_text)?;
    serde_json::from_str(&json_str).map_err(|e| format!("Failed to parse JSON response: {}", e))
}

async fn generate_mind_dump_analysis_anthropic(
    mind_dump_content: &str,
    config: &LLMConfig,
) -> Result<GenerateMindDumpAnalysisResponse, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;
    let model = map_anthropic_model(&config.anthropic_model);

    let card_summaries = extract_card_summaries_with_tags()?;
    let prompt = get_mind_dump_analysis_prompt().replace("{CARDS_SIMPLIFIED}", &card_summaries);
    let full_prompt = format!("{}\n\n{}", prompt, mind_dump_content);

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &config.anthropic_api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&json!({
            "model": model,
            "max_tokens": 1024,
            "messages": [{ "role": "user", "content": full_prompt }]
        }))
        .send()
        .await
        .map_err(|e| format!("Anthropic request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Anthropic API error: {}", response.status()));
    }

    let data: Value = response.json().await.map_err(|e| format!("Failed to parse Anthropic response: {}", e))?;
    let response_text = data.get("content").and_then(|v| v.as_array()).and_then(|arr| arr.get(0))
        .and_then(|content| content.get("text")).and_then(|v| v.as_str())
        .ok_or("Invalid Anthropic response format")?;

    let json_str = extract_json_from_response(response_text)?;
    serde_json::from_str(&json_str).map_err(|e| format!("Failed to parse JSON response: {}", e))
}
