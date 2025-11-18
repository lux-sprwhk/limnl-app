/// Shared blocker patterns framework for analyzing psychological and cognitive patterns
/// that create life blocks. Used in discovery chat and mind dump analysis.

// The blocker patterns text - this is the single source of truth
pub const BLOCKER_PATTERNS_TEXT: &str = r#"Epistemological Obstacles:
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
- personalization_bias: Excessive self-blame"#;

/// Build blocker patterns framework for discovery chat (with context about card guidance)
pub fn get_blocker_patterns_framework() -> String {
    format!(
        r#"
SUPPORTING FRAMEWORK - BLOCKER PATTERNS:
You are trained to recognize psychological and cognitive patterns that create life blocks. Use these as supporting analytical lenses ONLY when they illuminate what the card is already pointing to:

{}

Only surface these patterns when they genuinely deepen understanding of what the card is revealing—never force connections or use them as a substitute for the card's wisdom."#,
        BLOCKER_PATTERNS_TEXT
    )
}

/// Build blocker patterns section for mind dump analysis
pub fn get_blocker_patterns_section() -> String {
    format!(
        r#"## Blocker Patterns:

Analyze the mind dump for psychological and cognitive patterns that may be creating blocks. Use these patterns as analytical lenses:

{}"#,
        BLOCKER_PATTERNS_TEXT
    )
}

/// Blocker patterns list for mind dump analysis (formatted for JSON response)
/// This is the same as BLOCKER_PATTERNS_TEXT - using the shared constant
pub fn get_blocker_patterns_list() -> &'static str {
    BLOCKER_PATTERNS_TEXT
}

/// List of all available blocker pattern identifiers
pub const BLOCKER_PATTERN_IDS: &[&str] = &[
    // Epistemological Obstacles
    "substantialist_thinking",
    "obstacle_of_experience",
    "verbal_obstacle",
    "unitary_knowledge",
    "pragmatic_knowledge",
    "quantitative_obstacle",
    "animistic_thinking",
    "mythical_valorization",
    "circular_reasoning",
    "false_precision",
    "cognitive_rigidity",
    "avoidance_pattern",
    // Cognitive Biases
    "confirmation_bias",
    "dunning_kruger_effect",
    "sunk_cost_fallacy",
    "choice_overload_paralysis",
    // Attachment Patterns
    "avoidant_attachment_block",
    "anxious_attachment_block",
    "disorganized_attachment_block",
    // Psychological Reactance
    "autonomy_threat_response",
    // Cognitive Distortions
    "catastrophic_thinking",
    "all_or_nothing_thinking",
    "mental_filtering",
    "personalization_bias",
];

