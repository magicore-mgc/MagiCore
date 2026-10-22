//! Flip one verified CAS generation to promoted.
//! Đổi một CAS generation đã xác minh sang promoted.
use crate::database::database::{errors::CasGenerationError, types::Database};
use rusqlite::params;

impl Database {
    /// Flip the verified staging row and require exactly one affected row.
    /// (Đổi row staging đã xác minh và bắt buộc đúng một row bị tác động.)
    pub(in crate::database::database::generation_lifecycle::promote::promote_impl) fn flip_cas_generation_to_promoted(
        &self,
        project_root: &str,
        generation: i64,
    ) -> std::result::Result<(), CasGenerationError> {
        let flipped = self
            .conn
            .execute(
                "UPDATE cas_generations SET state = 'promoted'
                 WHERE project_root = ?1 AND generation = ?2 AND state = 'staging'",
                params![project_root, generation],
            )
            .map_err(|e| CasGenerationError::Io(e.to_string()))?;
        if flipped != 1 {
            return Err(CasGenerationError::Io(format!(
                "promote flip touched {flipped} row(s), expected exactly 1 — \
                 transaction rolled back"
            )));
        }
        Ok(())
    }
}
