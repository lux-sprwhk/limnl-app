use rusqlite::{Connection, Result as SqlResult};
use std::path::PathBuf;
use std::sync::Mutex;
use directories::ProjectDirs;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db_path = Self::get_database_path()?;

        // Ensure the parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(&db_path)?;
        let db = Database {
            conn: Mutex::new(conn),
        };

        db.init_schema()?;
        Ok(db)
    }

    fn get_database_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let proj_dirs = ProjectDirs::from("com", "limnl", "limnl-journal")
            .ok_or("Failed to determine project directories")?;

        let data_dir = proj_dirs.data_dir();
        Ok(data_dir.join("dreams.db"))
    }

    fn init_schema(&self) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
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
            )",
            [],
        )?;

        // Create indexes for better query performance
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_dreams_date_occurred ON dreams(date_occurred)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_dreams_created_at ON dreams(created_at)",
            [],
        )?;

        Ok(())
    }

    pub fn get_connection(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().unwrap()
    }
}
