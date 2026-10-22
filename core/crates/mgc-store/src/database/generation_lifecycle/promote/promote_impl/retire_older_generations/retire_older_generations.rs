//! Retire obsolete older-generation CAS claims in order.
//! Nghỉ hưu claim CAS generation cũ theo thứ tự.
use crate::database::database::{errors::CasGenerationError, types::Database};
use rusqlite::params;

impl Database {
    /// Retire unprotected older claims and collapse older promoted generations.
    /// (Nghỉ hưu claim cũ không được bảo vệ và gộp generation promoted cũ.)
    pub(in crate::database::database::generation_lifecycle::promote::promote_impl) fn retire_older_cas_generation_claims(
        &self,
        project_root: &str,
        generation: i64,
    ) -> std::result::Result<(), CasGenerationError> {
        // Keep newer and staging generations; retire only older unprotected rows.
        // (Giữ generation mới hơn và đang staging; chỉ nghỉ hưu row cũ không được bảo vệ.)
        self.conn
            .execute(
                "DELETE FROM cas_blob_refs WHERE project_root = ?1
             AND generation < ?2
             AND NOT EXISTS (
                 SELECT 1 FROM cas_blob_refs t
                 WHERE t.project_root = ?1 AND t.generation = ?2
                   AND t.hash = cas_blob_refs.hash
             )
             AND NOT EXISTS (
                 SELECT 1 FROM cas_generations s
                 WHERE s.project_root = ?1 AND s.generation = cas_blob_refs.generation
                   AND s.state = 'staging'
             )",
                params![project_root, generation],
            )
            .map_err(|e| CasGenerationError::Io(e.to_string()))?;

        // Collapse surviving older promoted claims into the current token.
        // (Gộp claim promoted cũ còn lại vào token hiện tại.)
        self.conn
            .execute(
                "DELETE FROM cas_blob_refs WHERE project_root = ?1 AND generation < ?2
             AND generation IN (
                 SELECT generation FROM cas_generations
                 WHERE project_root = ?1 AND state = 'promoted'
             )",
                params![project_root, generation],
            )
            .map_err(|e| CasGenerationError::Io(e.to_string()))?;
        self.conn
            .execute(
                "DELETE FROM cas_generations
             WHERE project_root = ?1 AND generation < ?2 AND state = 'promoted'",
                params![project_root, generation],
            )
            .map_err(|e| CasGenerationError::Io(e.to_string()))?;
        Ok(())
    }
}
