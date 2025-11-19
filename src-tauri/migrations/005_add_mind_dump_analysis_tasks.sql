-- Add mind dump analysis tasks table
-- Stores actionable tasks extracted from mind dump analysis

CREATE TABLE IF NOT EXISTS mind_dump_analysis_tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mind_dump_analysis_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (mind_dump_analysis_id) REFERENCES mind_dump_analysis(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_mind_dump_analysis_tasks_analysis_id ON mind_dump_analysis_tasks(mind_dump_analysis_id);

