use super::models::{CreateDreamCreativePromptsInput, DreamCreativePrompts};
use super::Database;
use chrono::Utc;
use rusqlite::{params, Result as SqlResult};

impl Database {
    pub fn create_dream_creative_prompts(&self, input: CreateDreamCreativePromptsInput) -> SqlResult<DreamCreativePrompts> {
        let now = Utc::now();
        let conn = self.get_connection();

        // Check if creative prompts already exist for this analysis (inline query to avoid nested lock)
        let existing_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM dream_creative_prompts WHERE dream_analysis_id = ?1",
            params![input.dream_analysis_id],
            |row| row.get(0),
        )?;

        if existing_count > 0 {
            // Delete existing creative prompts before creating new ones
            conn.execute(
                "DELETE FROM dream_creative_prompts WHERE dream_analysis_id = ?1",
                params![input.dream_analysis_id],
            )?;
        }

        conn.execute(
            "INSERT INTO dream_creative_prompts (dream_analysis_id, image_prompts, music_prompts, story_prompts, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                input.dream_analysis_id,
                input.image_prompts,
                input.music_prompts,
                input.story_prompts,
                now.to_rfc3339(),
                now.to_rfc3339(),
            ],
        )?;

        let id = conn.last_insert_rowid();

        Ok(DreamCreativePrompts {
            id: Some(id),
            dream_analysis_id: input.dream_analysis_id,
            image_prompts: input.image_prompts,
            music_prompts: input.music_prompts,
            story_prompts: input.story_prompts,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn get_dream_creative_prompts(&self, dream_analysis_id: i64) -> SqlResult<Option<DreamCreativePrompts>> {
        let conn = self.get_connection();

        let mut stmt = conn.prepare(
            "SELECT id, dream_analysis_id, image_prompts, music_prompts, story_prompts, created_at, updated_at
             FROM dream_creative_prompts WHERE dream_analysis_id = ?1",
        )?;

        let prompts = stmt.query_row(params![dream_analysis_id], |row| {
            Ok(DreamCreativePrompts {
                id: Some(row.get(0)?),
                dream_analysis_id: row.get(1)?,
                image_prompts: row.get(2)?,
                music_prompts: row.get(3)?,
                story_prompts: row.get(4)?,
                created_at: row.get::<_, String>(5)?.parse().unwrap(),
                updated_at: row.get::<_, String>(6)?.parse().unwrap(),
            })
        });

        match prompts {
            Ok(p) => Ok(Some(p)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn delete_dream_creative_prompts(&self, dream_analysis_id: i64) -> SqlResult<bool> {
        let conn = self.get_connection();

        let rows_affected = conn.execute(
            "DELETE FROM dream_creative_prompts WHERE dream_analysis_id = ?1",
            params![dream_analysis_id],
        )?;

        Ok(rows_affected > 0)
    }
}
