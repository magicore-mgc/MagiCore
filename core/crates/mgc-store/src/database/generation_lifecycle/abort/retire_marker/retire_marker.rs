//! Retire the staging marker and require exactly one affected row.
//! Nghỉ hưu staging marker và bắt buộc đúng một row bị tác động.
use super::super::super::super::types::Database;
use crate::database::database::errors::CasGenerationError;
use rusqlite::params;

impl Database {
    /// Delete the staging marker and detect an unexpected state change.
    /// (Xóa staging marker và phát hiện state thay đổi ngoài dự kiến.)
    pub(in crate::database::database::generation_lifecycle::abort) fn retire_cas_generation_marker(
        &self,
        project_root: &str,
        generation: i64,
    ) -> std::result::Result<(), CasGenerationError> {
        let retired = self
            .conn
            .execute(
                "DELETE FROM cas_generations
                     WHERE project_root = ?1 AND generation = ?2 AND state = 'staging'",
                params![project_root, generation],
            )
            .map_err(|e| CasGenerationError::Io(e.to_string()))?;
        if retired != 1 {
            return Err(CasGenerationError::Io(format!(
                "abort retire touched {retired} row(s), expected exactly 1 — \
                     transaction rolled back"
            )));
        }
        Ok(())
    }
}
