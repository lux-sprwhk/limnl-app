-- Add mood_tags column to mind_dumps table
-- Stores AI-generated mood tags as JSON array string

ALTER TABLE mind_dumps ADD COLUMN mood_tags TEXT;

