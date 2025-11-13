-- Initial schema for Limnl
-- Creates all core tables: dreams, bugs, mind_dumps, cards, and relationship tables

-- Dreams table
CREATE TABLE IF NOT EXISTS dreams (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date_recorded TEXT NOT NULL,
    date_occurred TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    emotions_tags TEXT,
    sleep_quality INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_dreams_date_occurred ON dreams(date_occurred);
CREATE INDEX IF NOT EXISTS idx_dreams_created_at ON dreams(created_at);

-- Dream analyses table for cached analysis results
CREATE TABLE IF NOT EXISTS dream_analyses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    dream_id INTEGER NOT NULL UNIQUE,
    themes_patterns TEXT NOT NULL,
    emotional_analysis TEXT NOT NULL,
    narrative_summary TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (dream_id) REFERENCES dreams(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_dream_analyses_dream_id ON dream_analyses(dream_id);

-- Dream analysis cards junction table
CREATE TABLE IF NOT EXISTS dream_analysis_cards (
    dream_analysis_id INTEGER NOT NULL,
    card_id INTEGER NOT NULL,
    relevance_note TEXT,
    created_at TEXT NOT NULL,
    PRIMARY KEY (dream_analysis_id, card_id),
    FOREIGN KEY (dream_analysis_id) REFERENCES dream_analyses(id) ON DELETE CASCADE,
    FOREIGN KEY (card_id) REFERENCES cards(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_dream_analysis_cards_analysis_id ON dream_analysis_cards(dream_analysis_id);
CREATE INDEX IF NOT EXISTS idx_dream_analysis_cards_card_id ON dream_analysis_cards(card_id);

-- Dream creative prompts table for AI-generated creative prompts
CREATE TABLE IF NOT EXISTS dream_creative_prompts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    dream_analysis_id INTEGER NOT NULL UNIQUE,
    image_prompts TEXT NOT NULL,
    music_prompts TEXT NOT NULL,
    story_prompts TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (dream_analysis_id) REFERENCES dream_analyses(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_dream_creative_prompts_analysis_id ON dream_creative_prompts(dream_analysis_id);

-- Bugs table
CREATE TABLE IF NOT EXISTS bugs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    cards_drawn TEXT,
    conversation_history TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    resolved_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_bugs_status ON bugs(status);
CREATE INDEX IF NOT EXISTS idx_bugs_created_at ON bugs(created_at);

-- Mind dumps table
CREATE TABLE IF NOT EXISTS mind_dumps (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT,
    content TEXT NOT NULL,
    word_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_mind_dumps_created_at ON mind_dumps(created_at);

-- Cards table
CREATE TABLE IF NOT EXISTS cards (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL
);

-- Bug-cards junction table
CREATE TABLE IF NOT EXISTS bug_cards (
    bug_id INTEGER NOT NULL,
    card_id INTEGER NOT NULL,
    position INTEGER,
    created_at TEXT NOT NULL,
    PRIMARY KEY (bug_id, card_id),
    FOREIGN KEY (bug_id) REFERENCES bugs(id) ON DELETE CASCADE,
    FOREIGN KEY (card_id) REFERENCES cards(id)
);

CREATE INDEX IF NOT EXISTS idx_bug_cards_card_id ON bug_cards(card_id);
CREATE INDEX IF NOT EXISTS idx_bug_cards_bug_id ON bug_cards(bug_id);
