-- Add recurring dream and lucid dreaming tracking fields
-- These fields enable users to track patterns in their dreams

-- Add new columns to dreams table
ALTER TABLE dreams ADD COLUMN is_recurring INTEGER DEFAULT 0;
ALTER TABLE dreams ADD COLUMN last_occurrence_period TEXT;
ALTER TABLE dreams ADD COLUMN is_lucid INTEGER DEFAULT 0;

-- Note: SQLite uses INTEGER for booleans (0 = false, 1 = true)
-- last_occurrence_period accepts: 'today', 'yesterday', 'last_week',
-- 'few_weeks_ago', 'last_month', 'months_ago', 'last_year'
