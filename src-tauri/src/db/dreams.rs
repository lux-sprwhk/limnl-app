use super::models::{CreateDreamInput, Dream, UpdateDreamInput};
use super::Database;
use chrono::Utc;
use rusqlite::{params, Result as SqlResult};

impl Database {
    pub fn create_dream(&self, input: CreateDreamInput) -> SqlResult<Dream> {
        let conn = self.get_connection();
        let now = Utc::now();

        conn.execute(
            "INSERT INTO dreams (date_recorded, date_occurred, title, content, emotions_tags, sleep_quality, is_recurring, last_occurrence_period, is_lucid, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                now.to_rfc3339(),
                input.date_occurred.to_rfc3339(),
                input.title,
                input.content,
                input.emotions_tags,
                input.sleep_quality,
                input.is_recurring,
                input.last_occurrence_period,
                input.is_lucid,
                now.to_rfc3339(),
                now.to_rfc3339(),
            ],
        )?;

        let id = conn.last_insert_rowid();

        Ok(Dream {
            id: Some(id),
            date_recorded: now,
            date_occurred: input.date_occurred,
            title: input.title,
            content: input.content,
            emotions_tags: input.emotions_tags,
            sleep_quality: input.sleep_quality,
            is_recurring: input.is_recurring,
            last_occurrence_period: input.last_occurrence_period,
            is_lucid: input.is_lucid,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn get_dream(&self, id: i64) -> SqlResult<Option<Dream>> {
        let conn = self.get_connection();

        let mut stmt = conn.prepare(
            "SELECT id, date_recorded, date_occurred, title, content, emotions_tags, sleep_quality, is_recurring, last_occurrence_period, is_lucid, created_at, updated_at
             FROM dreams WHERE id = ?1",
        )?;

        let dream = stmt.query_row(params![id], |row| {
            Ok(Dream {
                id: Some(row.get(0)?),
                date_recorded: row.get::<_, String>(1)?.parse().unwrap(),
                date_occurred: row.get::<_, String>(2)?.parse().unwrap(),
                title: row.get(3)?,
                content: row.get(4)?,
                emotions_tags: row.get(5)?,
                sleep_quality: row.get(6)?,
                is_recurring: row.get::<_, Option<i32>>(7)?.map(|v| v != 0),
                last_occurrence_period: row.get(8)?,
                is_lucid: row.get::<_, Option<i32>>(9)?.map(|v| v != 0),
                created_at: row.get::<_, String>(10)?.parse().unwrap(),
                updated_at: row.get::<_, String>(11)?.parse().unwrap(),
            })
        });

        match dream {
            Ok(d) => Ok(Some(d)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn list_dreams(&self, limit: Option<i64>, offset: Option<i64>) -> SqlResult<Vec<Dream>> {
        let conn = self.get_connection();

        let query = format!(
            "SELECT id, date_recorded, date_occurred, title, content, emotions_tags, sleep_quality, is_recurring, last_occurrence_period, is_lucid, created_at, updated_at
             FROM dreams
             ORDER BY date_occurred DESC
             LIMIT {} OFFSET {}",
            limit.unwrap_or(100),
            offset.unwrap_or(0)
        );

        let mut stmt = conn.prepare(&query)?;

        let dreams = stmt
            .query_map([], |row| {
                Ok(Dream {
                    id: Some(row.get(0)?),
                    date_recorded: row.get::<_, String>(1)?.parse().unwrap(),
                    date_occurred: row.get::<_, String>(2)?.parse().unwrap(),
                    title: row.get(3)?,
                    content: row.get(4)?,
                    emotions_tags: row.get(5)?,
                    sleep_quality: row.get(6)?,
                    is_recurring: row.get::<_, Option<i32>>(7)?.map(|v| v != 0),
                    last_occurrence_period: row.get(8)?,
                    is_lucid: row.get::<_, Option<i32>>(9)?.map(|v| v != 0),
                    created_at: row.get::<_, String>(10)?.parse().unwrap(),
                    updated_at: row.get::<_, String>(11)?.parse().unwrap(),
                })
            })?
            .collect::<SqlResult<Vec<Dream>>>()?;

        Ok(dreams)
    }

    pub fn update_dream(&self, input: UpdateDreamInput) -> SqlResult<Option<Dream>> {
        let conn = self.get_connection();

        // First, get the existing dream
        let existing = self.get_dream(input.id)?;
        if existing.is_none() {
            return Ok(None);
        }

        let mut existing = existing.unwrap();
        let now = Utc::now();

        // Update only provided fields
        if let Some(date_occurred) = input.date_occurred {
            existing.date_occurred = date_occurred;
        }
        if let Some(title) = input.title {
            existing.title = title;
        }
        if let Some(content) = input.content {
            existing.content = content;
        }
        if let Some(emotions_tags) = input.emotions_tags {
            existing.emotions_tags = Some(emotions_tags);
        }
        if let Some(sleep_quality) = input.sleep_quality {
            existing.sleep_quality = Some(sleep_quality);
        }
        if let Some(is_recurring) = input.is_recurring {
            existing.is_recurring = Some(is_recurring);
        }
        if let Some(last_occurrence_period) = input.last_occurrence_period {
            existing.last_occurrence_period = Some(last_occurrence_period);
        }
        if let Some(is_lucid) = input.is_lucid {
            existing.is_lucid = Some(is_lucid);
        }

        existing.updated_at = now;

        conn.execute(
            "UPDATE dreams
             SET date_occurred = ?1, title = ?2, content = ?3, emotions_tags = ?4, sleep_quality = ?5, is_recurring = ?6, last_occurrence_period = ?7, is_lucid = ?8, updated_at = ?9
             WHERE id = ?10",
            params![
                existing.date_occurred.to_rfc3339(),
                existing.title,
                existing.content,
                existing.emotions_tags,
                existing.sleep_quality,
                existing.is_recurring,
                existing.last_occurrence_period,
                existing.is_lucid,
                existing.updated_at.to_rfc3339(),
                input.id,
            ],
        )?;

        Ok(Some(existing))
    }

    pub fn delete_dream(&self, id: i64) -> SqlResult<bool> {
        let conn = self.get_connection();

        let rows_affected = conn.execute("DELETE FROM dreams WHERE id = ?1", params![id])?;

        Ok(rows_affected > 0)
    }

    pub fn search_dreams(&self, query: &str) -> SqlResult<Vec<Dream>> {
        let conn = self.get_connection();

        let search_pattern = format!("%{}%", query);

        let mut stmt = conn.prepare(
            "SELECT id, date_recorded, date_occurred, title, content, emotions_tags, sleep_quality, is_recurring, last_occurrence_period, is_lucid, created_at, updated_at
             FROM dreams
             WHERE title LIKE ?1 OR content LIKE ?1
             ORDER BY date_occurred DESC",
        )?;

        let dreams = stmt
            .query_map(params![search_pattern], |row| {
                Ok(Dream {
                    id: Some(row.get(0)?),
                    date_recorded: row.get::<_, String>(1)?.parse().unwrap(),
                    date_occurred: row.get::<_, String>(2)?.parse().unwrap(),
                    title: row.get(3)?,
                    content: row.get(4)?,
                    emotions_tags: row.get(5)?,
                    sleep_quality: row.get(6)?,
                    is_recurring: row.get::<_, Option<i32>>(7)?.map(|v| v != 0),
                    last_occurrence_period: row.get(8)?,
                    is_lucid: row.get::<_, Option<i32>>(9)?.map(|v| v != 0),
                    created_at: row.get::<_, String>(10)?.parse().unwrap(),
                    updated_at: row.get::<_, String>(11)?.parse().unwrap(),
                })
            })?
            .collect::<SqlResult<Vec<Dream>>>()?;

        Ok(dreams)
    }
}
