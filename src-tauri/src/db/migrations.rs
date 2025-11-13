use rusqlite::{Connection, Result as SqlResult, params};

/// List of all migrations in order
/// Add new migrations to the end of this array
const MIGRATIONS: &[&str] = &[
    include_str!("../../migrations/001_initial.sql"),
    include_str!("../../migrations/002_add_dream_metadata.sql"),
];

/// Get the current schema version from the database
/// Returns 0 if no migrations have been applied yet
fn get_schema_version(conn: &Connection) -> SqlResult<i32> {
    conn.query_row(
        "SELECT version FROM schema_version ORDER BY version DESC LIMIT 1",
        [],
        |row| row.get(0)
    ).or(Ok(0))
}

/// Run all pending migrations
/// This is called on app startup to ensure the database schema is up to date
pub fn run_migrations(conn: &Connection) -> SqlResult<()> {
    // Create version tracking table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY,
            applied_at INTEGER NOT NULL
        )",
        [],
    )?;

    let current_version = get_schema_version(conn)?;

    // Run any migrations newer than current version
    for (idx, migration) in MIGRATIONS.iter().enumerate().skip(current_version as usize) {
        let version = (idx + 1) as i32;

        println!("Applying migration {}/{}...", version, MIGRATIONS.len());

        // Execute the migration SQL
        // Note: We use execute_batch which executes each statement separately.
        // For ALTER TABLE ADD COLUMN, if the column already exists, SQLite will return
        // an error. We catch and ignore "duplicate column" errors to make migrations
        // more defensive against manual schema changes.
        match conn.execute_batch(migration) {
            Ok(_) => {
                // All statements succeeded
            }
            Err(e) => {
                // Check if it's a "duplicate column" error
                let error_msg = e.to_string().to_lowercase();
                if error_msg.contains("duplicate column") || error_msg.contains("already exists") {
                    println!("Warning: Some columns may already exist in migration {}. Continuing...", version);
                    // Continue - the migration system will still record it as applied
                } else {
                    // Re-throw other errors
                    return Err(e);
                }
            }
        }

        // Record that this migration has been applied
        let timestamp = chrono::Utc::now().timestamp();
        conn.execute(
            "INSERT INTO schema_version (version, applied_at) VALUES (?1, ?2)",
            params![version, timestamp],
        )?;

        println!("Migration {} applied successfully", version);
    }

    if current_version == MIGRATIONS.len() as i32 {
        println!("Database schema is up to date (version {})", current_version);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_migrations_run_successfully() {
        let conn = Connection::open_in_memory().unwrap();

        // Run migrations
        run_migrations(&conn).unwrap();

        // Verify schema version was recorded
        let version: i32 = conn.query_row(
            "SELECT version FROM schema_version ORDER BY version DESC LIMIT 1",
            [],
            |row| row.get(0)
        ).unwrap();

        assert_eq!(version, MIGRATIONS.len() as i32);
    }

    #[test]
    fn test_migrations_are_idempotent() {
        let conn = Connection::open_in_memory().unwrap();

        // Run migrations twice
        run_migrations(&conn).unwrap();
        run_migrations(&conn).unwrap();

        // Should still only have one version record per migration
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM schema_version",
            [],
            |row| row.get(0)
        ).unwrap();

        assert_eq!(count, MIGRATIONS.len() as i32);
    }

    #[test]
    fn test_tables_created() {
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();

        // Verify all tables exist
        let tables = vec![
            "dreams",
            "bugs",
            "mind_dumps",
            "cards",
            "bug_cards",
            "dream_analyses",
            "dream_analysis_cards",
            "dream_creative_prompts",
        ];

        for table in tables {
            let exists: i32 = conn.query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                params![table],
                |row| row.get(0)
            ).unwrap();

            assert_eq!(exists, 1, "Table {} should exist", table);
        }
    }

    #[test]
    fn test_upgrade_from_existing_schema() {
        // Simulate an existing database created by the old init_schema method
        // (no schema_version table, but tables already exist)
        let conn = Connection::open_in_memory().unwrap();

        // Create some tables manually (simulating old schema)
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dreams (
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
            CREATE TABLE IF NOT EXISTS cards (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                created_at TEXT NOT NULL
            );
            "
        ).unwrap();

        // Insert some test data
        conn.execute(
            "INSERT INTO dreams (date_recorded, date_occurred, title, content, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
            params!["2025-01-01", "2025-01-01", "Test Dream", "Content", "2025-01-01T00:00:00Z", "2025-01-01T00:00:00Z"]
        ).unwrap();

        // Now run migrations (simulating upgrade)
        run_migrations(&conn).unwrap();

        // Verify schema_version was created and populated
        let version: i32 = conn.query_row(
            "SELECT version FROM schema_version ORDER BY version DESC LIMIT 1",
            [],
            |row| row.get(0)
        ).unwrap();
        assert_eq!(version, MIGRATIONS.len() as i32);

        // Verify existing data is preserved
        let dream_count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM dreams",
            [],
            |row| row.get(0)
        ).unwrap();
        assert_eq!(dream_count, 1, "Existing data should be preserved");

        // Verify all tables now exist (including new ones from migration)
        let tables = vec![
            "dreams",
            "bugs",
            "mind_dumps",
            "cards",
            "bug_cards",
            "dream_analyses",
            "dream_analysis_cards",
            "dream_creative_prompts",
        ];

        for table in tables {
            let exists: i32 = conn.query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                params![table],
                |row| row.get(0)
            ).unwrap();
            assert_eq!(exists, 1, "Table {} should exist after upgrade", table);
        }
    }

    #[test]
    fn test_migration_002_adds_dream_metadata_columns() {
        let conn = Connection::open_in_memory().unwrap();
        
        // Run all migrations
        run_migrations(&conn).unwrap();

        // Verify new columns exist in dreams table
        let columns = vec![
            "is_recurring",
            "last_occurrence_period",
            "is_lucid",
        ];

        for column in columns {
            let exists: i32 = conn.query_row(
                "SELECT COUNT(*) FROM pragma_table_info('dreams') WHERE name = ?1",
                params![column],
                |row| row.get(0)
            ).unwrap();
            assert_eq!(exists, 1, "Column {} should exist in dreams table", column);
        }

        // Verify columns allow NULL (no NOT NULL constraint)
        let is_recurring_nullable: i32 = conn.query_row(
            "SELECT \"notnull\" FROM pragma_table_info('dreams') WHERE name = 'is_recurring'",
            [],
            |row| row.get(0)
        ).unwrap();
        assert_eq!(is_recurring_nullable, 0, "is_recurring should allow NULL");

        let is_lucid_nullable: i32 = conn.query_row(
            "SELECT \"notnull\" FROM pragma_table_info('dreams') WHERE name = 'is_lucid'",
            [],
            |row| row.get(0)
        ).unwrap();
        assert_eq!(is_lucid_nullable, 0, "is_lucid should allow NULL");
    }
}
