use serde_json::Value;

/// Extract just card names and core meanings from cards.json (for dream analysis)
pub fn extract_card_summaries() -> Result<String, String> {
    let cards_json = include_str!("../../../src/cards.json");
    let cards_data: Value = serde_json::from_str(cards_json)
        .map_err(|e| format!("Failed to parse cards.json: {}", e))?;

    let cards_array = cards_data
        .get("cards")
        .and_then(|v| v.as_array())
        .ok_or("Invalid cards.json structure")?;

    let mut summaries = Vec::new();
    for card in cards_array {
        let name = card.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
        let meaning = card.get("core_meaning").and_then(|v| v.as_str()).unwrap_or("No meaning");
        summaries.push(format!("- {}: {}", name, meaning));
    }

    Ok(summaries.join("\n"))
}

/// Extract card names, meanings, and tags (more efficient for mind dump analysis)
pub fn extract_card_summaries_with_tags() -> Result<String, String> {
    let cards_json = include_str!("../../../src/cards.json");
    let cards_data: Value = serde_json::from_str(cards_json)
        .map_err(|e| format!("Failed to parse cards.json: {}", e))?;

    let cards_array = cards_data
        .get("cards")
        .and_then(|v| v.as_array())
        .ok_or("Invalid cards.json structure")?;

    let mut summaries = Vec::new();
    for card in cards_array {
        let name = card.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
        let meaning = card.get("core_meaning").and_then(|v| v.as_str()).unwrap_or("No meaning");
        let tags = card.get("tags")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|t| t.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_default();

        summaries.push(format!("- {}: {} [tags: {}]", name, meaning, tags));
    }

    Ok(summaries.join("\n"))
}
