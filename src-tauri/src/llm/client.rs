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

const CARD_COMMENTARY_PROMPT: &str = r#"You are a guide helping someone discover what's really bothering them through reflective card-based inquiry.

A person is exploring a bug or issue in their {life_area} area. They've drawn a card that might help them understand what's blocking them.

Card: {card_name}
Question: {card_question}
Meaning: {card_meaning}

Provide a brief, insightful commentary (1-2 short sentences) on how this card's meaning might relate to issues in their {life_area} area. Help them see potential connections without being prescriptive.

Keep the commentary concise and to the point.
Respond with ONLY the commentary, nothing else."#;

const MULTIPLE_CARDS_COMMENTARY_PROMPT: &str = r#"You are a guide helping someone discover what's really bothering them through reflective card-based inquiry.

A person is exploring a bug or issue in their {life_area} area. They've drawn three cards that might help them understand what's blocking them.

For each card below, provide a brief, insightful commentary (1-2 short sentences) on how that card's meaning might relate to issues in their {life_area} area.

Cards:
{cards_list}

Respond with a JSON object where keys are card IDs (as strings) and values are the commentaries. Example format:
{{"1": "commentary for card 1", "2": "commentary for card 2", "3": "commentary for card 3"}}

Respond with ONLY the JSON object, nothing else."#;

const DISCOVERY_CHAT_SYSTEM_PROMPT: &str = r#"You are a compassionate and insightful guide helping someone discover what's really bothering them through reflective conversation.

The person is using a card-based discovery process to uncover a "bug" (a recurring issue or pattern) in their {life_area} area. They've drawn the "{card_name}" card to help guide the exploration.

PRIMARY LENS - THE CARD:
The card is your main analytical tool. Always ground your responses in the card's meaning and how it relates to what they're sharing.

Card context:
- Card: {card_name}
- Question: {card_question}
- Meaning: {card_meaning}

Use the card's meaning as the primary framework for understanding their situation. Ask questions that help them see connections between the card's wisdom and their lived experience.

Your role is to:
- Ask thoughtful, open-ended questions rooted in the card's meaning
- Listen carefully to what they share and reflect it back through the card's lens
- Help them connect their feelings and experiences to the card's insights
- Guide them toward insights about what might be blocking them
- Be warm, non-judgmental, and genuinely curious
- Keep responses concise and focused (1-3 sentences typically)

SUPPORTING FRAMEWORK - BLOCKER PATTERNS:
You are trained to recognize psychological and cognitive patterns that create life blocks. Use these as supporting analytical lenses ONLY when they illuminate what the card is already pointing to:

Epistemological Obstacles:
- substantialist_thinking: Treating dynamic processes as fixed traits
- obstacle_of_experience: Over-reliance on past experience
- verbal_obstacle: Getting trapped in language/metaphors
- unitary_knowledge: Resistance to complexity
- pragmatic_knowledge: Quick-fix seeking over deep understanding
- quantitative_obstacle: Over-focus on metrics
- animistic_thinking: External attribution patterns
- mythical_valorization: Idealization of approaches
- circular_reasoning: Self-reinforcing thought loops
- false_precision: Pseudo-accuracy masking uncertainty
- cognitive_rigidity: Difficulty shifting perspectives
- avoidance_pattern: Systematic topic avoidance

Cognitive Biases:
- confirmation_bias: Seeking confirming information
- dunning_kruger_effect: Overconfidence in low-competence areas
- sunk_cost_fallacy: Continuing due to past investment
- choice_overload_paralysis: Decision avoidance

Attachment Patterns:
- avoidant_attachment_block: Emotional discomfort
- anxious_attachment_block: Validation seeking
- disorganized_attachment_block: Chaotic engagement

Psychological Reactance:
- autonomy_threat_response: Resistance to perceived control

Cognitive Distortions:
- catastrophic_thinking: Worst-case scenario focus
- all_or_nothing_thinking: Black-and-white patterns
- mental_filtering: Negative focus
- personalization_bias: Excessive self-blame

Only surface these patterns when they genuinely deepen understanding of what the card is revealing—never force connections or use them as a substitute for the card's wisdom.

Remember: Your goal is to help them discover their own insights through the card's guidance, not to tell them what's wrong."#;

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
    config: &LLMConfig,
) -> Result<String, String> {
    match config.provider {
        LLMProvider::Disabled => Err("LLM is disabled".to_string()),
        LLMProvider::Ollama => chat_with_history_ollama(user_message, messages, card_name, card_question, card_meaning, card_insights, life_area, user_name, zodiac_sign, mbti_type, config).await,
        LLMProvider::OpenAI => chat_with_history_openai(user_message, messages, card_name, card_question, card_meaning, card_insights, life_area, user_name, zodiac_sign, mbti_type, config).await,
        LLMProvider::Anthropic => chat_with_history_anthropic(user_message, messages, card_name, card_question, card_meaning, card_insights, life_area, user_name, zodiac_sign, mbti_type, config).await,
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
    let json_start = response_text.find('{').ok_or("No JSON object found in response")?;
    let json_end = response_text.rfind('}').ok_or("No JSON object found in response")?;
    let json_str = &response_text[json_start..=json_end];

    serde_json::from_str(json_str)
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
    let json_start = response_text.find('{').ok_or("No JSON object found in response")?;
    let json_end = response_text.rfind('}').ok_or("No JSON object found in response")?;
    let json_str = &response_text[json_start..=json_end];

    serde_json::from_str(json_str)
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
    let json_start = response_text.find('{').ok_or("No JSON object found in response")?;
    let json_end = response_text.rfind('}').ok_or("No JSON object found in response")?;
    let json_str = &response_text[json_start..=json_end];

    serde_json::from_str(json_str)
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

    let mut system_prompt = DISCOVERY_CHAT_SYSTEM_PROMPT
        .replace("{life_area}", life_area)
        .replace("{card_name}", card_name)
        .replace("{card_question}", card_question)
        .replace("{card_meaning}", card_meaning);
    
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
    config: &LLMConfig,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let model = map_openai_model(&config.openai_model);

    let mut system_prompt = DISCOVERY_CHAT_SYSTEM_PROMPT
        .replace("{life_area}", life_area)
        .replace("{card_name}", card_name)
        .replace("{card_question}", card_question)
        .replace("{card_meaning}", card_meaning);
    
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
    config: &LLMConfig,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let model = map_anthropic_model(&config.anthropic_model);

    let mut system_prompt = DISCOVERY_CHAT_SYSTEM_PROMPT
        .replace("{life_area}", life_area)
        .replace("{card_name}", card_name)
        .replace("{card_question}", card_question)
        .replace("{card_meaning}", card_meaning);
    
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
