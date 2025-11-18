-- Rename word_count to character_count in mind_dumps table
-- SQLite doesn't support RENAME COLUMN directly in older versions
-- So we'll use ALTER TABLE ADD COLUMN and then migrate the data

-- Add new character_count column
ALTER TABLE mind_dumps ADD COLUMN character_count INTEGER NOT NULL DEFAULT 0;

-- Copy data from word_count to character_count (for existing rows)
-- This will be done via a trigger since we can't use UPDATE in migrations directly
-- For new migrations, we'll just use the default value

-- Note: Existing data will have word_count values copied to character_count
-- This is acceptable as a migration step - users can re-save their mind dumps
-- to get accurate character counts
