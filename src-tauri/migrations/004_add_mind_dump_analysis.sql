-- Add mind dump analysis table
-- Stores LLM-generated card associations for mind dumps

CREATE TABLE IF NOT EXISTS mind_dump_analysis (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mind_dump_id INTEGER NOT NULL UNIQUE,
    created_at TEXT NOT NULL,
    FOREIGN KEY (mind_dump_id) REFERENCES mind_dumps(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_mind_dump_analysis_mind_dump_id ON mind_dump_analysis(mind_dump_id);

-- Mind dump analysis cards junction table (similar to dream_analysis_cards)
CREATE TABLE IF NOT EXISTS mind_dump_analysis_cards (
    mind_dump_analysis_id INTEGER NOT NULL,
    card_id INTEGER NOT NULL,
    relevance_note TEXT,
    created_at TEXT NOT NULL,
    PRIMARY KEY (mind_dump_analysis_id, card_id),
    FOREIGN KEY (mind_dump_analysis_id) REFERENCES mind_dump_analysis(id) ON DELETE CASCADE,
    FOREIGN KEY (card_id) REFERENCES cards(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_mind_dump_analysis_cards_analysis_id ON mind_dump_analysis_cards(mind_dump_analysis_id);
CREATE INDEX IF NOT EXISTS idx_mind_dump_analysis_cards_card_id ON mind_dump_analysis_cards(card_id);
