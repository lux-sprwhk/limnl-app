-- Add recurring dream and lucid dreaming tracking fields
-- These fields enable users to track patterns in their dreams
--
-- ASSUMES: dreams table exists from migration 001
-- ASSUMES: is_recurring, last_occurrence_period, is_lucid columns do NOT exist
--
-- Note: SQLite uses INTEGER for booleans (0 = false, 1 = true, NULL = unknown/not set)
-- last_occurrence_period accepts: 'today', 'yesterday', 'last_week',
-- 'few_weeks_ago', 'last_month', 'months_ago', 'last_year'
--
-- These columns allow NULL to match Rust Option<bool> and Option<String> types.
-- NULL means the field hasn't been set, which is different from false/empty.

-- Add new columns to dreams table (allow NULL to match Rust Option types)
-- SQLite 3.38.0+ supports IF NOT EXISTS, but we check schema first for compatibility
-- If columns already exist, these statements will be ignored by migration system
ALTER TABLE dreams ADD COLUMN is_recurring INTEGER;
ALTER TABLE dreams ADD COLUMN last_occurrence_period TEXT;
ALTER TABLE dreams ADD COLUMN is_lucid INTEGER;
