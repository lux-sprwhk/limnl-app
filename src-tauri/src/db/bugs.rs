use super::models::{Bug, CreateBugInput, UpdateBugInput};
use super::Database;
use chrono::Utc;
use rusqlite::{params, Result as SqlResult};

impl Database {
    pub fn create_bug(&self, input: CreateBugInput) -> SqlResult<Bug> {
        let conn = self.get_connection();
        let now = Utc::now();

        // Note: cards_drawn is kept for backward compatibility but should use bug_cards table instead
        conn.execute(
            "INSERT INTO bugs (title, description, status, cards_drawn, conversation_history, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                input.title,
                input.description,
                "active",
                input.cards_drawn,
                input.conversation_history,
                now.to_rfc3339(),
                now.to_rfc3339(),
            ],
        )?;

        let id = conn.last_insert_rowid();

        Ok(Bug {
            id: Some(id),
            title: input.title,
            description: input.description,
            status: "active".to_string(),
            cards_drawn: input.cards_drawn,
            conversation_history: input.conversation_history,
            created_at: now,
            updated_at: now,
            resolved_at: None,
        })
    }

    /// Create a bug with cards using the new card tables
    /// This is the preferred method for creating bugs with cards
    /// Card names must exist in the deck - will return error if not found
    pub fn create_bug_with_cards(
        &self,
        input: CreateBugInput,
        card_names: Vec<String>,
    ) -> SqlResult<Bug> {
        // Create the bug first
        let bug = self.create_bug(input)?;
        let bug_id = bug.id.unwrap();

        // Look up cards by name and link them to the bug
        for (position, card_name) in card_names.iter().enumerate() {
            // Get card by name - it must already exist in the deck
            let card = self.get_card_by_name(card_name)?
                .ok_or_else(|| rusqlite::Error::InvalidQuery)?;
            let card_id = card.id.unwrap();

            self.link_card_to_bug(bug_id, card_id, Some((position + 1) as i32))?;
        }

        Ok(bug)
    }

    pub fn get_bug(&self, id: i64) -> SqlResult<Option<Bug>> {
        let conn = self.get_connection();

        let mut stmt = conn.prepare(
            "SELECT id, title, description, status, cards_drawn, conversation_history, created_at, updated_at, resolved_at
             FROM bugs WHERE id = ?1",
        )?;

        let bug = stmt.query_row(params![id], |row| {
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
        });

        match bug {
            Ok(b) => Ok(Some(b)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn list_bugs(&self, status: Option<String>) -> SqlResult<Vec<Bug>> {
        let conn = self.get_connection();

        let (query, params) = if let Some(status_filter) = status {
            (
                "SELECT id, title, description, status, cards_drawn, conversation_history, created_at, updated_at, resolved_at
                 FROM bugs
                 WHERE status = ?1
                 ORDER BY created_at DESC".to_string(),
                vec![status_filter],
            )
        } else {
            (
                "SELECT id, title, description, status, cards_drawn, conversation_history, created_at, updated_at, resolved_at
                 FROM bugs
                 ORDER BY created_at DESC".to_string(),
                vec![],
            )
        };

        let mut stmt = conn.prepare(&query)?;

        let bugs = stmt
            .query_map(rusqlite::params_from_iter(params.iter()), |row| {
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

    pub fn update_bug(&self, input: UpdateBugInput) -> SqlResult<Option<Bug>> {
        let conn = self.get_connection();

        // First, get the existing bug
        let existing = self.get_bug(input.id)?;
        if existing.is_none() {
            return Ok(None);
        }

        let mut existing = existing.unwrap();
        let now = Utc::now();

        // Update only provided fields
        if let Some(title) = input.title {
            existing.title = title;
        }
        if let Some(description) = input.description {
            existing.description = description;
        }
        if let Some(status) = input.status {
            existing.status = status;
        }
        if let Some(cards_drawn) = input.cards_drawn {
            existing.cards_drawn = Some(cards_drawn);
        }
        if let Some(conversation_history) = input.conversation_history {
            existing.conversation_history = Some(conversation_history);
        }
        if let Some(resolved_at) = input.resolved_at {
            existing.resolved_at = Some(resolved_at);
        }

        existing.updated_at = now;

        conn.execute(
            "UPDATE bugs
             SET title = ?1, description = ?2, status = ?3, cards_drawn = ?4, conversation_history = ?5, updated_at = ?6, resolved_at = ?7
             WHERE id = ?8",
            params![
                existing.title,
                existing.description,
                existing.status,
                existing.cards_drawn,
                existing.conversation_history,
                existing.updated_at.to_rfc3339(),
                existing.resolved_at.map(|dt| dt.to_rfc3339()),
                input.id,
            ],
        )?;

        Ok(Some(existing))
    }

    pub fn delete_bug(&self, id: i64) -> SqlResult<bool> {
        let conn = self.get_connection();

        let rows_affected = conn.execute("DELETE FROM bugs WHERE id = ?1", params![id])?;

        Ok(rows_affected > 0)
    }
}
