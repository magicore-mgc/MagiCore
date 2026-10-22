//! Validate the token state before abort mutations.
//! Xác minh state token trước mutation abort.
use super::super::super::super::types::Database;
use crate::database::database::errors::CasGenerationError;
use rusqlite::{OptionalExtension, params};

impl Database {
    /// Return false for a missing token; reject promoted generations.
    /// (Trả false khi token không còn; từ chối generation đã promoted.)
    pub(in crate::database::database::generation_lifecycle::abort) fn validate_cas_generation_abort_state(
        &self,
        project_root: &str,
        generation: i64,
    ) -> std::result::Result<bool, CasGenerationError> {
        let state: Option<String> = self
            .conn
            .query_row(
                "SELECT state FROM cas_generations
                     WHERE project_root = ?1 AND generation = ?2",
                params![project_root, generation],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|e| CasGenerationError::Io(e.to_string()))?;
        match state.as_deref() {
            Some("staging") => Ok(true),
            Some(_) => Err(CasGenerationError::AlreadyPromoted {
                project_root: project_root.to_string(),
                generation,
            }),
            None => {
                // Idempotent no-op when crash recovery retries a retired token.
                // (No-op idempotent khi phục hồi crash retry token đã nghỉ hưu.)
                Ok(false)
            }
        }
    }
}
