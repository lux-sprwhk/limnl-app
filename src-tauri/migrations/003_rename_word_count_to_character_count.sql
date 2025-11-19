-- Rename word_count to character_count in mind_dumps table
-- SQLite doesn't support DROP COLUMN in older versions, so we recreate the table
-- This is safe since there's no existing data to preserve

-- Step 1: Create new table with character_count instead of word_count
CREATE TABLE IF NOT EXISTS mind_dumps_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT,
    content TEXT NOT NULL,
    character_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Step 2: Copy any existing data (migrating word_count to character_count if needed)
-- Since we have no data, this is just for safety
INSERT INTO mind_dumps_new (id, title, content, character_count, created_at, updated_at)
SELECT id, title, content, COALESCE(word_count, 0), created_at, updated_at
FROM mind_dumps;

-- Step 3: Drop old table
DROP TABLE IF EXISTS mind_dumps;

-- Step 4: Rename new table to original name
ALTER TABLE mind_dumps_new RENAME TO mind_dumps;

-- Step 5: Recreate index
CREATE INDEX IF NOT EXISTS idx_mind_dumps_created_at ON mind_dumps(created_at);
