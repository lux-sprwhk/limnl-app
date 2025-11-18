use super::models::{MindDumpAnalysis, MindDumpAnalysisCard, MindDumpAnalysisWithCards, MindDumpAnalysisTask, MindDumpAnalysisWithCardsAndTasks};
use super::Database;
use chrono::Utc;
use rusqlite::{params, Result as SqlResult};

impl Database {
    /// Create a mind dump analysis entry
    pub fn create_mind_dump_analysis(&self, mind_dump_id: i64) -> SqlResult<MindDumpAnalysis> {
        let conn = self.get_connection();
        let now = Utc::now();

        conn.execute(
            "INSERT INTO mind_dump_analysis (mind_dump_id, blocker_patterns, created_at) VALUES (?1, ?2, ?3)",
            params![mind_dump_id, None::<String>, now.to_rfc3339()],
        )?;

        let id = conn.last_insert_rowid();

        Ok(MindDumpAnalysis {
            id: Some(id),
            mind_dump_id,
            blocker_patterns: None,
            created_at: now,
        })
    }

    /// Link a card to a mind dump analysis with a relevance note
    pub fn link_card_to_mind_dump_analysis(
        &self,
        mind_dump_analysis_id: i64,
        card_id: i64,
        relevance_note: Option<String>,
    ) -> SqlResult<()> {
        let conn = self.get_connection();
        let now = Utc::now();

        conn.execute(
            "INSERT INTO mind_dump_analysis_cards (mind_dump_analysis_id, card_id, relevance_note, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![mind_dump_analysis_id, card_id, relevance_note, now.to_rfc3339()],
        )?;

        Ok(())
    }

    /// Get mind dump analysis with associated cards
    pub fn get_mind_dump_analysis_with_cards(
        &self,
        mind_dump_id: i64,
    ) -> SqlResult<Option<MindDumpAnalysisWithCards>> {
        let conn = self.get_connection();

        // Get the analysis
        let mut stmt = conn.prepare(
            "SELECT id, mind_dump_id, blocker_patterns, created_at FROM mind_dump_analysis WHERE mind_dump_id = ?1",
        )?;

        let analysis = stmt.query_row(params![mind_dump_id], |row| {
            Ok(MindDumpAnalysis {
                id: Some(row.get(0)?),
                mind_dump_id: row.get(1)?,
                blocker_patterns: row.get(2)?,
                created_at: row.get::<_, String>(3)?.parse().unwrap(),
            })
        });

        let analysis = match analysis {
            Ok(a) => a,
            Err(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
            Err(e) => return Err(e),
        };

        // Get the associated cards
        let mut stmt = conn.prepare(
            "SELECT mac.mind_dump_analysis_id, mac.card_id, c.name, mac.relevance_note, mac.created_at
             FROM mind_dump_analysis_cards mac
             JOIN cards c ON mac.card_id = c.id
             WHERE mac.mind_dump_analysis_id = ?1",
        )?;

        let cards = stmt
            .query_map(params![analysis.id.unwrap()], |row| {
                Ok(MindDumpAnalysisCard {
                    mind_dump_analysis_id: row.get(0)?,
                    card_id: row.get(1)?,
                    card_name: row.get(2)?,
                    relevance_note: row.get(3)?,
                    created_at: row.get::<_, String>(4)?.parse().unwrap(),
                })
            })?
            .collect::<SqlResult<Vec<MindDumpAnalysisCard>>>()?;

        Ok(Some(MindDumpAnalysisWithCards { analysis, cards }))
    }

    /// Update blocker patterns for a mind dump analysis
    pub fn update_mind_dump_analysis_blocker_patterns(
        &self,
        mind_dump_analysis_id: i64,
        blocker_patterns: Option<String>,
    ) -> SqlResult<()> {
        let conn = self.get_connection();

        conn.execute(
            "UPDATE mind_dump_analysis SET blocker_patterns = ?1 WHERE id = ?2",
            params![blocker_patterns, mind_dump_analysis_id],
        )?;

        Ok(())
    }

    /// Create a task for a mind dump analysis
    pub fn create_mind_dump_analysis_task(
        &self,
        mind_dump_analysis_id: i64,
        title: String,
        description: Option<String>,
    ) -> SqlResult<MindDumpAnalysisTask> {
        let conn = self.get_connection();
        let now = Utc::now();

        conn.execute(
            "INSERT INTO mind_dump_analysis_tasks (mind_dump_analysis_id, title, description, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![mind_dump_analysis_id, title, description, now.to_rfc3339()],
        )?;

        let id = conn.last_insert_rowid();

        Ok(MindDumpAnalysisTask {
            id: Some(id),
            mind_dump_analysis_id,
            title,
            description,
            created_at: now,
        })
    }

    /// Get mind dump analysis with associated cards and tasks
    pub fn get_mind_dump_analysis_with_cards_and_tasks(
        &self,
        mind_dump_id: i64,
    ) -> SqlResult<Option<MindDumpAnalysisWithCardsAndTasks>> {
        let conn = self.get_connection();

        // Get the analysis
        let mut stmt = conn.prepare(
            "SELECT id, mind_dump_id, blocker_patterns, created_at FROM mind_dump_analysis WHERE mind_dump_id = ?1",
        )?;

        let analysis = stmt.query_row(params![mind_dump_id], |row| {
            Ok(MindDumpAnalysis {
                id: Some(row.get(0)?),
                mind_dump_id: row.get(1)?,
                blocker_patterns: row.get(2)?,
                created_at: row.get::<_, String>(3)?.parse().unwrap(),
            })
        });

        let analysis = match analysis {
            Ok(a) => a,
            Err(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
            Err(e) => return Err(e),
        };

        let analysis_id = analysis.id.unwrap();

        // Get the associated cards
        let mut stmt = conn.prepare(
            "SELECT mac.mind_dump_analysis_id, mac.card_id, c.name, mac.relevance_note, mac.created_at
             FROM mind_dump_analysis_cards mac
             JOIN cards c ON mac.card_id = c.id
             WHERE mac.mind_dump_analysis_id = ?1",
        )?;

        let cards = stmt
            .query_map(params![analysis_id], |row| {
                Ok(MindDumpAnalysisCard {
                    mind_dump_analysis_id: row.get(0)?,
                    card_id: row.get(1)?,
                    card_name: row.get(2)?,
                    relevance_note: row.get(3)?,
                    created_at: row.get::<_, String>(4)?.parse().unwrap(),
                })
            })?
            .collect::<SqlResult<Vec<MindDumpAnalysisCard>>>()?;

        // Get the associated tasks
        let mut stmt = conn.prepare(
            "SELECT id, mind_dump_analysis_id, title, description, created_at
             FROM mind_dump_analysis_tasks
             WHERE mind_dump_analysis_id = ?1
             ORDER BY created_at ASC",
        )?;

        let tasks = stmt
            .query_map(params![analysis_id], |row| {
                Ok(MindDumpAnalysisTask {
                    id: Some(row.get(0)?),
                    mind_dump_analysis_id: row.get(1)?,
                    title: row.get(2)?,
                    description: row.get(3)?,
                    created_at: row.get::<_, String>(4)?.parse().unwrap(),
                })
            })?
            .collect::<SqlResult<Vec<MindDumpAnalysisTask>>>()?;

        Ok(Some(MindDumpAnalysisWithCardsAndTasks { analysis, cards, tasks }))
    }
}
