use rusqlite::{Connection, Result as SqlResult, params};

/// List of all migrations in order
/// Add new migrations to the end of this array
const MIGRATIONS: &[&str] = &[
    include_str!("../../migrations/001_initial.sql"),
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
        conn.execute_batch(migration)?;

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

        // Verify key tables exist
        let tables = vec!["dreams", "bugs", "mind_dumps", "cards", "bug_cards"];

        for table in tables {
            let exists: i32 = conn.query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                params![table],
                |row| row.get(0)
            ).unwrap();

            assert_eq!(exists, 1, "Table {} should exist", table);
        }
    }
}
