use rusqlite::{Connection, Result as SqlResult};
use std::path::PathBuf;
use std::sync::Mutex;
use directories::ProjectDirs;
use serde::Deserialize;

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

        // Create dream_analyses table for cached analysis results
        conn.execute(
            "CREATE TABLE IF NOT EXISTS dream_analyses (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                dream_id INTEGER NOT NULL UNIQUE,
                themes_patterns TEXT NOT NULL,
                emotional_analysis TEXT NOT NULL,
                narrative_summary TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (dream_id) REFERENCES dreams(id) ON DELETE CASCADE
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_dream_analyses_dream_id ON dream_analyses(dream_id)",
            [],
        )?;

        // Create dream_analysis_cards junction table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS dream_analysis_cards (
                dream_analysis_id INTEGER NOT NULL,
                card_id INTEGER NOT NULL,
                relevance_note TEXT,
                created_at TEXT NOT NULL,
                PRIMARY KEY (dream_analysis_id, card_id),
                FOREIGN KEY (dream_analysis_id) REFERENCES dream_analyses(id) ON DELETE CASCADE,
                FOREIGN KEY (card_id) REFERENCES cards(id) ON DELETE CASCADE
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_dream_analysis_cards_analysis_id ON dream_analysis_cards(dream_analysis_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_dream_analysis_cards_card_id ON dream_analysis_cards(card_id)",
            [],
        )?;

        // Create dream_creative_prompts table for AI-generated creative prompts
        conn.execute(
            "CREATE TABLE IF NOT EXISTS dream_creative_prompts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                dream_analysis_id INTEGER NOT NULL UNIQUE,
                image_prompts TEXT NOT NULL,
                music_prompts TEXT NOT NULL,
                story_prompts TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (dream_analysis_id) REFERENCES dream_analyses(id) ON DELETE CASCADE
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_dream_creative_prompts_analysis_id ON dream_creative_prompts(dream_analysis_id)",
            [],
        )?;

        // Create bugs table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS bugs (
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
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_bugs_status ON bugs(status)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_bugs_created_at ON bugs(created_at)",
            [],
        )?;

        // Create mind_dumps table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS mind_dumps (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT,
                content TEXT NOT NULL,
                word_count INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_mind_dumps_created_at ON mind_dumps(created_at)",
            [],
        )?;

        // Create cards table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS cards (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        // Create bug_cards junction table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS bug_cards (
                bug_id INTEGER NOT NULL,
                card_id INTEGER NOT NULL,
                position INTEGER,
                created_at TEXT NOT NULL,
                PRIMARY KEY (bug_id, card_id),
                FOREIGN KEY (bug_id) REFERENCES bugs(id) ON DELETE CASCADE,
                FOREIGN KEY (card_id) REFERENCES cards(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_bug_cards_card_id ON bug_cards(card_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_bug_cards_bug_id ON bug_cards(bug_id)",
            [],
        )?;

        // Migration: Add notes column to bugs table if it doesn't exist
        // This is a simple migration for existing databases
        let _ = conn.execute("ALTER TABLE bugs ADD COLUMN notes TEXT", []);
        // Ignore error if column already exists

        // Release the connection lock before seeding
        drop(conn);

        // Seed cards if not already seeded
        self.seed_cards_from_json()?;

        Ok(())
    }

    fn seed_cards_from_json(&self) -> SqlResult<()> {
        use super::models::CreateCardInput;

        // Check if cards already exist
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM cards", [], |row| row.get(0))?;
        drop(conn);

        if count > 0 {
            // Cards already seeded
            return Ok(());
        }

        // Load cards from JSON file
        #[derive(Deserialize)]
        struct CardData {
            name: String,
        }

        #[derive(Deserialize)]
        struct CardsJson {
            cards: Vec<CardData>,
        }

        let cards_json = include_str!("../../cards.json");
        let cards_data: CardsJson = serde_json::from_str(cards_json)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

        // Seed each card
        for card_data in cards_data.cards {
            self.create_card_internal(CreateCardInput {
                name: card_data.name,
            })?;
        }

        Ok(())
    }

    pub fn get_connection(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().unwrap()
    }

    pub fn get_database_path_public() -> Result<PathBuf, Box<dyn std::error::Error>> {
        Self::get_database_path()
    }

    pub fn backup_database(&self, destination: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let source = Self::get_database_path()?;
        std::fs::copy(&source, destination)?;
        Ok(())
    }
}
