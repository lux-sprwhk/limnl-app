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

        let cards_json = include_str!("../../../src/cards.json");
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

    #[cfg(test)]
    /// Create a Database instance from an existing connection (for testing only)
    pub fn from_connection(conn: Connection) -> Self {
        Database {
            conn: Mutex::new(conn),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::{Connection, params};
    use crate::db::migrations;

    #[test]
    fn test_cards_json_is_embedded_and_parseable() {
        // Test that cards.json is accessible from the new path
        let cards_json = include_str!("../../../src/cards.json");
        
        // Verify it's not empty
        assert!(!cards_json.is_empty(), "cards.json should not be empty");
        
        // Verify it's valid JSON
        let cards_data: serde_json::Value = serde_json::from_str(cards_json)
            .expect("cards.json should be valid JSON");
        
        // Verify it has the expected structure
        assert!(cards_data.get("cards").is_some(), "cards.json should have a 'cards' array");
        
        let cards_array = cards_data.get("cards")
            .and_then(|v| v.as_array())
            .expect("cards should be an array");
        
        assert!(!cards_array.is_empty(), "cards array should not be empty");
        
        // Verify at least one card has required fields
        let first_card = cards_array[0].as_object().expect("card should be an object");
        assert!(first_card.get("name").is_some(), "card should have a 'name' field");
    }

    #[test]
    fn test_cards_seeded_from_json() {
        // Create in-memory database
        let conn = Connection::open_in_memory().unwrap();
        
        // Run migrations first
        migrations::run_migrations(&conn).unwrap();
        
        // Create database instance with this connection using test-only constructor
        let db = Database::from_connection(conn);
        
        // Seed cards
        db.seed_cards_from_json().unwrap();
        
        // Verify cards were seeded
        let conn = db.get_connection();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM cards", [], |row| row.get(0)).unwrap();
        drop(conn);
        
        assert!(count > 0, "Cards should be seeded from JSON");
        
        // Verify specific card exists (e.g., "Delivery Driver" from cards.json)
        let conn = db.get_connection();
        let exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM cards WHERE name = ?1",
            params!["Delivery Driver"],
            |row| row.get(0)
        ).unwrap();
        drop(conn);
        
        assert_eq!(exists, 1, "Delivery Driver card should exist");
    }
}
