pub const MULTIPLE_CARDS_COMMENTARY_PROMPT: &str = r#"You are a guide helping someone discover what's really bothering them through reflective card-based inquiry.

A person is exploring a bug or issue in their {life_area} area. They've drawn three cards that might help them understand what's blocking them.

For each card below, provide a brief, insightful commentary (1-2 short sentences) on how that card's meaning might relate to issues in their {life_area} area.

Cards:
{cards_list}

Respond with a JSON object where keys are card IDs (as strings) and values are the commentaries. Example format:
{{"1": "commentary for card 1", "2": "commentary for card 2", "3": "commentary for card 3"}}

Respond with ONLY the JSON object, nothing else."#;
