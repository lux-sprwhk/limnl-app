-- Add blocker_patterns column to mind_dump_analysis table
-- Stores AI-identified blocker patterns as JSON array string

ALTER TABLE mind_dump_analysis ADD COLUMN blocker_patterns TEXT;

