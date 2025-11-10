use super::models::{CreateDreamAnalysisInput, DreamAnalysis, DreamAnalysisCard, DreamAnalysisWithCards};
use super::Database;
use chrono::Utc;
use rusqlite::{params, Result as SqlResult};

impl Database {
    pub fn create_dream_analysis(&self, input: CreateDreamAnalysisInput) -> SqlResult<DreamAnalysis> {
        let conn = self.get_connection();
        let now = Utc::now();

        // Check if analysis already exists for this dream
        let existing = self.get_dream_analysis(input.dream_id)?;
        if existing.is_some() {
            // Delete existing analysis before creating new one
            conn.execute(
                "DELETE FROM dream_analyses WHERE dream_id = ?1",
                params![input.dream_id],
            )?;
        }

        conn.execute(
            "INSERT INTO dream_analyses (dream_id, themes_patterns, emotional_analysis, narrative_summary, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                input.dream_id,
                input.themes_patterns,
                input.emotional_analysis,
                input.narrative_summary,
                now.to_rfc3339(),
                now.to_rfc3339(),
            ],
        )?;

        let id = conn.last_insert_rowid();

        Ok(DreamAnalysis {
            id: Some(id),
            dream_id: input.dream_id,
            themes_patterns: input.themes_patterns,
            emotional_analysis: input.emotional_analysis,
            narrative_summary: input.narrative_summary,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn get_dream_analysis(&self, dream_id: i64) -> SqlResult<Option<DreamAnalysis>> {
        let conn = self.get_connection();

        let mut stmt = conn.prepare(
            "SELECT id, dream_id, themes_patterns, emotional_analysis, narrative_summary, created_at, updated_at
             FROM dream_analyses WHERE dream_id = ?1",
        )?;

        let analysis = stmt.query_row(params![dream_id], |row| {
            Ok(DreamAnalysis {
                id: Some(row.get(0)?),
                dream_id: row.get(1)?,
                themes_patterns: row.get(2)?,
                emotional_analysis: row.get(3)?,
                narrative_summary: row.get(4)?,
                created_at: row.get::<_, String>(5)?.parse().unwrap(),
                updated_at: row.get::<_, String>(6)?.parse().unwrap(),
            })
        });

        match analysis {
            Ok(a) => Ok(Some(a)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn delete_dream_analysis(&self, dream_id: i64) -> SqlResult<bool> {
        let conn = self.get_connection();

        let rows_affected = conn.execute(
            "DELETE FROM dream_analyses WHERE dream_id = ?1",
            params![dream_id],
        )?;

        Ok(rows_affected > 0)
    }

    pub fn link_card_to_dream_analysis(
        &self,
        dream_analysis_id: i64,
        card_id: i64,
        relevance_note: Option<String>,
    ) -> SqlResult<()> {
        let conn = self.get_connection();
        let now = Utc::now();

        conn.execute(
            "INSERT OR REPLACE INTO dream_analysis_cards (dream_analysis_id, card_id, relevance_note, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                dream_analysis_id,
                card_id,
                relevance_note,
                now.to_rfc3339(),
            ],
        )?;

        Ok(())
    }

    pub fn get_dream_analysis_with_cards(&self, dream_id: i64) -> SqlResult<Option<DreamAnalysisWithCards>> {
        let analysis = self.get_dream_analysis(dream_id)?;

        if let Some(analysis) = analysis {
            let analysis_id = analysis.id.unwrap_or(0);
            let cards = self.get_dream_analysis_cards(analysis_id)?;

            Ok(Some(DreamAnalysisWithCards { analysis, cards }))
        } else {
            Ok(None)
        }
    }

    pub fn get_dream_analysis_cards(&self, dream_analysis_id: i64) -> SqlResult<Vec<DreamAnalysisCard>> {
        let conn = self.get_connection();

        let mut stmt = conn.prepare(
            "SELECT dac.dream_analysis_id, dac.card_id, c.name, dac.relevance_note, dac.created_at
             FROM dream_analysis_cards dac
             JOIN cards c ON dac.card_id = c.id
             WHERE dac.dream_analysis_id = ?1
             ORDER BY dac.created_at ASC",
        )?;

        let cards = stmt
            .query_map(params![dream_analysis_id], |row| {
                Ok(DreamAnalysisCard {
                    dream_analysis_id: row.get(0)?,
                    card_id: row.get(1)?,
                    card_name: row.get(2)?,
                    relevance_note: row.get(3)?,
                    created_at: row.get::<_, String>(4)?.parse().unwrap(),
                })
            })?
            .collect::<SqlResult<Vec<DreamAnalysisCard>>>()?;

        Ok(cards)
    }

    pub fn clear_dream_analysis_cards(&self, dream_analysis_id: i64) -> SqlResult<()> {
        let conn = self.get_connection();

        conn.execute(
            "DELETE FROM dream_analysis_cards WHERE dream_analysis_id = ?1",
            params![dream_analysis_id],
        )?;

        Ok(())
    }
}
