use super::models::{BugCard, Bug, Card, CardWithCount, CreateCardInput};
use super::Database;
use chrono::Utc;
use rusqlite::{params, Result as SqlResult};

impl Database {
    /// Internal function to create a card (used for seeding the deck)
    /// Not exposed to users - the deck is fixed like a tarot deck
    pub(super) fn create_card_internal(&self, input: CreateCardInput) -> SqlResult<Card> {
        let conn = self.get_connection();
        let now = Utc::now();

        // Try to insert, if it fails due to unique constraint, fetch the existing card
        let result = conn.execute(
            "INSERT INTO cards (name, created_at) VALUES (?1, ?2)",
            params![input.name, now.to_rfc3339()],
        );

        match result {
            Ok(_) => {
                let id = conn.last_insert_rowid();
                Ok(Card {
                    id: Some(id),
                    name: input.name,
                    created_at: now,
                })
            }
            Err(rusqlite::Error::SqliteFailure(err, _))
                if err.code == rusqlite::ErrorCode::ConstraintViolation =>
            {
                // Card already exists, fetch it
                let mut stmt = conn.prepare("SELECT id, name, created_at FROM cards WHERE name = ?1")?;
                stmt.query_row(params![input.name], |row| {
                    Ok(Card {
                        id: Some(row.get(0)?),
                        name: row.get(1)?,
                        created_at: row.get::<_, String>(2)?.parse().unwrap(),
                    })
                })
            }
            Err(e) => Err(e),
        }
    }

    /// Get a card by ID
    pub fn get_card(&self, id: i64) -> SqlResult<Option<Card>> {
        let conn = self.get_connection();

        let mut stmt = conn.prepare("SELECT id, name, created_at FROM cards WHERE id = ?1")?;

        let card = stmt.query_row(params![id], |row| {
            Ok(Card {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                created_at: row.get::<_, String>(2)?.parse().unwrap(),
            })
        });

        match card {
            Ok(c) => Ok(Some(c)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Get a card by name
    pub fn get_card_by_name(&self, name: &str) -> SqlResult<Option<Card>> {
        let conn = self.get_connection();

        let mut stmt = conn.prepare("SELECT id, name, created_at FROM cards WHERE name = ?1")?;

        let card = stmt.query_row(params![name], |row| {
            Ok(Card {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                created_at: row.get::<_, String>(2)?.parse().unwrap(),
            })
        });

        match card {
            Ok(c) => Ok(Some(c)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// List all cards
    pub fn list_cards(&self) -> SqlResult<Vec<Card>> {
        let conn = self.get_connection();

        let mut stmt = conn.prepare(
            "SELECT id, name, created_at FROM cards ORDER BY name ASC"
        )?;

        let cards = stmt
            .query_map([], |row| {
                Ok(Card {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    created_at: row.get::<_, String>(2)?.parse().unwrap(),
                })
            })?
            .collect::<SqlResult<Vec<Card>>>()?;

        Ok(cards)
    }

    /// List all cards sorted by bug count (most used first)
    pub fn list_cards_by_usage(&self) -> SqlResult<Vec<CardWithCount>> {
        let conn = self.get_connection();

        let mut stmt = conn.prepare(
            "SELECT c.id, c.name, COUNT(bc.bug_id) as bug_count, c.created_at
             FROM cards c
             LEFT JOIN bug_cards bc ON c.id = bc.card_id
             GROUP BY c.id, c.name, c.created_at
             ORDER BY bug_count DESC, c.name ASC"
        )?;

        let cards = stmt
            .query_map([], |row| {
                Ok(CardWithCount {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    bug_count: row.get(2)?,
                    created_at: row.get::<_, String>(3)?.parse().unwrap(),
                })
            })?
            .collect::<SqlResult<Vec<CardWithCount>>>()?;

        Ok(cards)
    }

    /// Link a card to a bug
    pub fn link_card_to_bug(&self, bug_id: i64, card_id: i64, position: Option<i32>) -> SqlResult<BugCard> {
        let conn = self.get_connection();
        let now = Utc::now();

        conn.execute(
            "INSERT OR REPLACE INTO bug_cards (bug_id, card_id, position, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![bug_id, card_id, position, now.to_rfc3339()],
        )?;

        Ok(BugCard {
            bug_id,
            card_id,
            position,
            created_at: now,
        })
    }

    /// Get all cards for a specific bug
    pub fn get_bug_cards(&self, bug_id: i64) -> SqlResult<Vec<Card>> {
        let conn = self.get_connection();

        let mut stmt = conn.prepare(
            "SELECT c.id, c.name, c.created_at
             FROM cards c
             INNER JOIN bug_cards bc ON c.id = bc.card_id
             WHERE bc.bug_id = ?1
             ORDER BY bc.position ASC, c.name ASC"
        )?;

        let cards = stmt
            .query_map(params![bug_id], |row| {
                Ok(Card {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    created_at: row.get::<_, String>(2)?.parse().unwrap(),
                })
            })?
            .collect::<SqlResult<Vec<Card>>>()?;

        Ok(cards)
    }

    /// Remove a card from a bug
    pub fn unlink_card_from_bug(&self, bug_id: i64, card_id: i64) -> SqlResult<bool> {
        let conn = self.get_connection();

        let rows_affected = conn.execute(
            "DELETE FROM bug_cards WHERE bug_id = ?1 AND card_id = ?2",
            params![bug_id, card_id],
        )?;

        Ok(rows_affected > 0)
    }

    /// Remove all cards from a bug
    pub fn clear_bug_cards(&self, bug_id: i64) -> SqlResult<()> {
        let conn = self.get_connection();

        conn.execute(
            "DELETE FROM bug_cards WHERE bug_id = ?1",
            params![bug_id],
        )?;

        Ok(())
    }

    /// Get all bugs for a specific card
    pub fn get_card_bugs(&self, card_id: i64) -> SqlResult<Vec<Bug>> {
        let conn = self.get_connection();

        let mut stmt = conn.prepare(
            "SELECT b.id, b.title, b.description, b.status, b.cards_drawn, b.conversation_history, b.created_at, b.updated_at, b.resolved_at
             FROM bugs b
             INNER JOIN bug_cards bc ON b.id = bc.bug_id
             WHERE bc.card_id = ?1
             ORDER BY b.created_at DESC"
        )?;

        let bugs = stmt
            .query_map(params![card_id], |row| {
                Ok(Bug {
                    id: Some(row.get(0)?),
                    title: row.get(1)?,
                    description: row.get(2)?,
                    status: row.get(3)?,
                    cards_drawn: row.get(4)?,
                    conversation_history: row.get(5)?,
                    created_at: row.get::<_, String>(6)?.parse().unwrap(),
                    updated_at: row.get::<_, String>(7)?.parse().unwrap(),
                    resolved_at: row.get::<_, Option<String>>(8)?.map(|s| s.parse().unwrap()),
                })
            })?
            .collect::<SqlResult<Vec<Bug>>>()?;

        Ok(bugs)
    }

    // Note: No delete_card function - cards are permanent like a tarot deck
}
