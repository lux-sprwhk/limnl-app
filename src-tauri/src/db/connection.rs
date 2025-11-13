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

        // Run migrations before wrapping in Mutex
        super::migrations::run_migrations(&conn)?;

        let db = Database {
            conn: Mutex::new(conn),
        };

        // Seed cards after migrations
        db.seed_cards_from_json()?;

        Ok(db)
    }

    fn get_database_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let proj_dirs = ProjectDirs::from("com", "limnl", "limnl-journal")
            .ok_or("Failed to determine project directories")?;

        let data_dir = proj_dirs.data_dir();
        Ok(data_dir.join("dreams.db"))
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
