use super::blocker_patterns::get_blocker_patterns_section;
use std::sync::OnceLock;

// Build the prompt using the shared blocker patterns framework
fn build_mind_dump_analysis_prompt() -> String {
    format!(
        r#"You are analyzing a mind dump to find 1-2 matching cards, extract actionable tasks, identify mood tags, and detect blocker patterns.

## Card Deck:

{{CARDS_SIMPLIFIED}}

## Available Mood Tags:

Choose from these mood tags that best describe the emotional state of the mind dump:
- angry, anxious, apathetic, calm, confused, content, disappointed, excited, fearful, frustrated, grateful, guilty, happy, hopeful, hype, indifferent, lonely, nostalgic, overwhelmed, peaceful, proud, relieved, sad, satisfied, stressed, thankful, worried, completion, accomplishment, determined, motivated, exhausted, energized, reflective, contemplative, uncertain, confident, insecure, optimistic, pessimistic, nostalgic, regretful, joyful, melancholic, serene, agitated, focused, scattered

Return valid JSON only:

```json
{{
  "relevant_cards": [
    {{
      "card_name": "Exact card name",
      "relevance_note": "One sentence why"
    }}
  ],
  "tasks": [
    {{
      "title": "Clear, actionable task title",
      "description": "Optional brief description or context"
    }}
  ],
  "mood_tags": ["mood1", "mood2", "mood3"],
  "blocker_patterns": ["pattern1", "pattern2"]
}}
```

Choose 1-2 cards max that match the mind dump's themes/emotions. Brief explanation.

Extract actionable tasks that the user can add to their task manager (reminders, Google Tasks, Todoist, etc.). Only include tasks that are:
- Specific and actionable (not vague thoughts or feelings)
- Something the user can actually do
- Clear enough to be understood out of context

Skip tasks if none are found. Return empty array if no actionable tasks exist.

Identify 1-3 mood tags that best capture the emotional tone of the mind dump. Select the most relevant tags from the list above. Return empty array if no clear mood can be determined.

{}

Identify 0-3 blocker patterns that are present in the mind dump. Only include patterns that genuinely illuminate what's blocking the person. Return empty array if no clear patterns are detected.

---

Mind dump:"#,
        get_blocker_patterns_section()
    )
}

// Cache the prompt for performance (built once, reused)
static MIND_DUMP_ANALYSIS_PROMPT_CACHE: OnceLock<String> = OnceLock::new();

/// Get the mind dump analysis prompt (uses shared blocker patterns framework)
pub fn get_mind_dump_analysis_prompt() -> &'static str {
    MIND_DUMP_ANALYSIS_PROMPT_CACHE.get_or_init(|| build_mind_dump_analysis_prompt())
}

// For backward compatibility - this now uses the shared framework
pub const MIND_DUMP_ANALYSIS_PROMPT: &str = "";
