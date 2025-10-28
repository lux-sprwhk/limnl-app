use super::models::{CreateMindDumpInput, MindDump, UpdateMindDumpInput};
use super::Database;
use chrono::Utc;
use rusqlite::{params, Result as SqlResult};

impl Database {
    pub fn create_mind_dump(&self, input: CreateMindDumpInput) -> SqlResult<MindDump> {
        let conn = self.get_connection();
        let now = Utc::now();

        conn.execute(
            "INSERT INTO mind_dumps (title, content, word_count, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                input.title,
                input.content,
                input.word_count,
                now.to_rfc3339(),
                now.to_rfc3339(),
            ],
        )?;

        let id = conn.last_insert_rowid();

        Ok(MindDump {
            id: Some(id),
            title: input.title,
            content: input.content,
            word_count: input.word_count,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn get_mind_dump(&self, id: i64) -> SqlResult<Option<MindDump>> {
        let conn = self.get_connection();

        let mut stmt = conn.prepare(
            "SELECT id, title, content, word_count, created_at, updated_at
             FROM mind_dumps WHERE id = ?1",
        )?;

        let entry = stmt.query_row(params![id], |row| {
            Ok(MindDump {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                content: row.get(2)?,
                word_count: row.get(3)?,
                created_at: row.get::<_, String>(4)?.parse().unwrap(),
                updated_at: row.get::<_, String>(5)?.parse().unwrap(),
            })
        });

        match entry {
            Ok(e) => Ok(Some(e)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn list_mind_dumps(&self, limit: Option<i64>, offset: Option<i64>) -> SqlResult<Vec<MindDump>> {
        let conn = self.get_connection();

        let query = format!(
            "SELECT id, title, content, word_count, created_at, updated_at
             FROM mind_dumps
             ORDER BY created_at DESC
             LIMIT {} OFFSET {}",
            limit.unwrap_or(100),
            offset.unwrap_or(0)
        );

        let mut stmt = conn.prepare(&query)?;

        let entries = stmt
            .query_map([], |row| {
                Ok(MindDump {
                    id: Some(row.get(0)?),
                    title: row.get(1)?,
                    content: row.get(2)?,
                    word_count: row.get(3)?,
                    created_at: row.get::<_, String>(4)?.parse().unwrap(),
                    updated_at: row.get::<_, String>(5)?.parse().unwrap(),
                })
            })?
            .collect::<SqlResult<Vec<MindDump>>>()?;

        Ok(entries)
    }

    pub fn update_mind_dump(&self, input: UpdateMindDumpInput) -> SqlResult<Option<MindDump>> {
        let conn = self.get_connection();

        // First, get the existing entry
        let existing = self.get_mind_dump(input.id)?;
        if existing.is_none() {
            return Ok(None);
        }

        let mut existing = existing.unwrap();
        let now = Utc::now();

        // Update only provided fields
        if let Some(title) = input.title {
            existing.title = Some(title);
        }
        if let Some(content) = input.content {
            existing.content = content;
        }
        if let Some(word_count) = input.word_count {
            existing.word_count = word_count;
        }

        existing.updated_at = now;

        conn.execute(
            "UPDATE mind_dumps
             SET title = ?1, content = ?2, word_count = ?3, updated_at = ?4
             WHERE id = ?5",
            params![
                existing.title,
                existing.content,
                existing.word_count,
                existing.updated_at.to_rfc3339(),
                input.id,
            ],
        )?;

        Ok(Some(existing))
    }

    pub fn delete_mind_dump(&self, id: i64) -> SqlResult<bool> {
        let conn = self.get_connection();

        let rows_affected = conn.execute("DELETE FROM mind_dumps WHERE id = ?1", params![id])?;

        Ok(rows_affected > 0)
    }

    pub fn search_mind_dumps(&self, query: &str) -> SqlResult<Vec<MindDump>> {
        let conn = self.get_connection();

        let search_pattern = format!("%{}%", query);

        let mut stmt = conn.prepare(
            "SELECT id, title, content, word_count, created_at, updated_at
             FROM mind_dumps
             WHERE title LIKE ?1 OR content LIKE ?1
             ORDER BY created_at DESC",
        )?;

        let entries = stmt
            .query_map(params![search_pattern], |row| {
                Ok(MindDump {
                    id: Some(row.get(0)?),
                    title: row.get(1)?,
                    content: row.get(2)?,
                    word_count: row.get(3)?,
                    created_at: row.get::<_, String>(4)?.parse().unwrap(),
                    updated_at: row.get::<_, String>(5)?.parse().unwrap(),
                })
            })?
            .collect::<SqlResult<Vec<MindDump>>>()?;

        Ok(entries)
    }
}
