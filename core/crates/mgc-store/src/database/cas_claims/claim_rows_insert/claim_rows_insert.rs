//! Insert a verified batch of CAS claim rows.
//! Insert một nhóm row claim CAS đã được xác minh.
use super::super::super::types::Database;
use crate::database::database::errors::CasGenerationError;
use rusqlite::params;

impl Database {
    /// Insert all requested hashes using the caller's active transaction.
    /// (Insert toàn bộ hash được yêu cầu bằng transaction hiện hành của caller.)
    pub(in crate::database::database::cas_claims) fn insert_cas_claim_rows(
        &self,
        project_root: &str,
        generation: i64,
        hashes: &[&str],
    ) -> std::result::Result<(), CasGenerationError> {
        let mut statement = self
            .conn
            .prepare_cached(
                "INSERT OR IGNORE INTO cas_blob_refs (project_root, generation, hash)
                     VALUES (?1, ?2, ?3)",
            )
            .map_err(|e| CasGenerationError::Io(format!("prepare claim insert: {e}")))?;
        for (index, hash) in hashes.iter().enumerate() {
            statement
                .execute(params![project_root, generation, hash])
                .map_err(|e| {
                    CasGenerationError::Io(format!(
                        "insert claim {}/{}: {e}",
                        index + 1,
                        hashes.len()
                    ))
                })?;
        }
        Ok(())
    }
}
