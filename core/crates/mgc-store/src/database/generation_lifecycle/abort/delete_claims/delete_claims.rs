//! Delete only the claim rows owned by the aborted generation.
//! Chỉ xóa các row claim thuộc generation bị abort.
use super::super::super::super::types::Database;
use crate::database::database::errors::CasGenerationError;
use rusqlite::params;

impl Database {
    /// Delete this token's claims inside the caller's active transaction.
    /// (Xóa claim của token trong transaction hiện hành của caller.)
    pub(in crate::database::database::generation_lifecycle::abort) fn delete_cas_generation_claims(
        &self,
        project_root: &str,
        generation: i64,
    ) -> std::result::Result<(), CasGenerationError> {
        self.conn
            .execute(
                "DELETE FROM cas_blob_refs WHERE project_root = ?1 AND generation = ?2",
                params![project_root, generation],
            )
            .map_err(|e| CasGenerationError::Io(e.to_string()))?;
        Ok(())
    }
}
