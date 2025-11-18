pub const MIND_DUMP_ANALYSIS_PROMPT: &str = r#"You are analyzing a mind dump to find 1-2 matching cards, extract actionable tasks, and identify mood tags.

## Card Deck:

{CARDS_SIMPLIFIED}

## Available Mood Tags:

Choose from these mood tags that best describe the emotional state of the mind dump:
- angry, anxious, apathetic, calm, confused, content, disappointed, excited, fearful, frustrated, grateful, guilty, happy, hopeful, hype, indifferent, lonely, nostalgic, overwhelmed, peaceful, proud, relieved, sad, satisfied, stressed, thankful, worried, completion, accomplishment, determined, motivated, exhausted, energized, reflective, contemplative, uncertain, confident, insecure, optimistic, pessimistic, nostalgic, regretful, joyful, melancholic, serene, agitated, focused, scattered

Return valid JSON only:

```json
{
  "relevant_cards": [
    {
      "card_name": "Exact card name",
      "relevance_note": "One sentence why"
    }
  ],
  "tasks": [
    {
      "title": "Clear, actionable task title",
      "description": "Optional brief description or context"
    }
  ],
  "mood_tags": ["mood1", "mood2", "mood3"]
}
```

Choose 1-2 cards max that match the mind dump's themes/emotions. Brief explanation.

Extract actionable tasks that the user can add to their task manager (reminders, Google Tasks, Todoist, etc.). Only include tasks that are:
- Specific and actionable (not vague thoughts or feelings)
- Something the user can actually do
- Clear enough to be understood out of context

Skip tasks if none are found. Return empty array if no actionable tasks exist.

Identify 1-3 mood tags that best capture the emotional tone of the mind dump. Select the most relevant tags from the list above. Return empty array if no clear mood can be determined.

---

Mind dump:"#;
