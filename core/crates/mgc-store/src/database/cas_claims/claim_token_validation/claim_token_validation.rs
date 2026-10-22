//! Validate staging-token ownership before a CAS claim write.
//! Xác minh token staging thuộc đúng project trước khi ghi CAS claim.
use super::super::super::types::Database;
use crate::database::database::errors::CasGenerationError;
use rusqlite::{OptionalExtension, params};

impl Database {
    /// Require an existing staging token for this project before insertion.
    /// (Bắt buộc token staging tồn tại và thuộc project trước khi insert.)
    pub(in crate::database::database::cas_claims) fn validate_cas_claim_staging_token(
        &self,
        project_root: &str,
        generation: i64,
    ) -> std::result::Result<(), CasGenerationError> {
        // The capability gate runs before writes while the caller holds the writer lock.
        // (Cổng năng lực chạy trước thao tác ghi khi caller đang giữ writer lock.)
        let state: Option<String> = self
            .conn
            .query_row(
                "SELECT state FROM cas_generations
                     WHERE project_root = ?1 AND generation = ?2",
                params![project_root, generation],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|e| CasGenerationError::Io(format!("read generation state: {e}")))?;
        match state.as_deref() {
            Some("staging") => Ok(()),
            Some(_) => Err(CasGenerationError::AlreadyPromoted {
                project_root: project_root.to_string(),
                generation,
            }),
            None => Err(CasGenerationError::UnknownToken {
                project_root: project_root.to_string(),
                generation,
            }),
        }
    }
}
