# Database Migrations

This directory contains SQL migration files for the Limnl application database.

## How It Works

- Migrations are numbered sequentially: `001_initial.sql`, `002_add_feature.sql`, etc.
- Each migration is applied exactly once in order
- The `schema_version` table tracks which migrations have been applied
- Migrations run automatically on app startup

## Adding a New Migration

1. Create a new file with the next sequential number:
   ```bash
   touch migrations/002_add_my_feature.sql
   ```

2. Write your SQL schema changes:
   ```sql
   -- migrations/002_add_my_feature.sql
   CREATE TABLE my_new_table (
       id INTEGER PRIMARY KEY,
       name TEXT NOT NULL
   );

   CREATE INDEX idx_my_table_name ON my_new_table(name);
   ```

3. Add the migration to the `MIGRATIONS` array in `src/db/migrations.rs`:
   ```rust
   const MIGRATIONS: &[&str] = &[
       include_str!("../../migrations/001_initial.sql"),
       include_str!("../../migrations/002_add_my_feature.sql"),  // Add this line
   ];
   ```

4. Test your migration:
   ```bash
   cargo test db::migrations::tests
   ```

5. The migration will be applied automatically next time the app starts

## Guidelines

- **Forward-only**: No rollback support (desktop app doesn't need it)
- **Idempotent SQL**: Use `IF NOT EXISTS` where possible for safety
- **One migration per feature**: Keep migrations focused and atomic
- **Test thoroughly**: Use `cargo test` to verify migrations work
- **Sequential numbering**: Always use the next sequential number

## Schema Version Tracking

The `schema_version` table tracks applied migrations:

```sql
CREATE TABLE schema_version (
    version INTEGER PRIMARY KEY,
    applied_at INTEGER NOT NULL  -- Unix timestamp
);
```

To check your current schema version:

```sql
SELECT version FROM schema_version ORDER BY version DESC LIMIT 1;
```

## Example Migration Workflow

```bash
# Create new migration file
touch migrations/003_add_tags.sql

# Edit the SQL
cat > migrations/003_add_tags.sql << 'EOF'
CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL
);

CREATE TABLE dream_tags (
    dream_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (dream_id, tag_id),
    FOREIGN KEY (dream_id) REFERENCES dreams(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);
EOF

# Add to MIGRATIONS array in src/db/migrations.rs
# Run tests
cargo test db::migrations::tests

# Done! Migration will run on next app start
```

## Upgrade Path for Existing Users

If you're upgrading from a version that used the old `init_schema` method (pre-migration system), the migration system will automatically handle the upgrade:

1. **On first launch after upgrade**: The migration runner detects that no `schema_version` table exists
2. **Safe execution**: The `001_initial.sql` migration uses `IF NOT EXISTS` clauses for all DDL statements
3. **Idempotent**: Existing tables and data are preserved - the migration only creates missing tables/columns
4. **Version tracking**: After running, `schema_version` will show version 1, indicating the initial migration has been applied

**What this means:**
- ✅ Your existing data is safe - no data loss
- ✅ The migration system will track future schema changes
- ✅ All tables will be created if they don't exist (for new installations)
- ✅ Existing tables remain unchanged (for upgrades)

**Note**: The migration system assumes that existing databases created by `init_schema` already have all the tables defined in `001_initial.sql`. If you had a database created before certain tables were added (e.g., before `dream_analyses` was introduced), those tables will be created automatically on upgrade.

## Troubleshooting

**Migration failed during development?**
- Delete your local database and let it rebuild from scratch
- Check SQL syntax in the migration file
- Ensure `IF NOT EXISTS` is used where appropriate

**Need to fix a migration?**
- If not yet released: fix the SQL file directly
- If already released: create a new migration to fix the issue

**Schema version out of sync?**
```sql
-- Reset schema version (development only!)
DELETE FROM schema_version;
```

## Why This Approach?

- **Simple**: Just SQL files and a small runner
- **No magic**: You see exactly what's happening
- **No dependencies**: Uses only rusqlite
- **Fast compile times**: No ORM overhead
- **Desktop-optimized**: No distributed coordination needed
- **Easy debugging**: Plain SQL you can run manually
