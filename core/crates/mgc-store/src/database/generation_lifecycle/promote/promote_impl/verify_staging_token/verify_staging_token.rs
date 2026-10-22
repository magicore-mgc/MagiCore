//! Verify a promotion token before any mutation.
//! Xác minh token promote trước mọi mutation.
use crate::database::database::{errors::CasGenerationError, types::Database};
use rusqlite::{OptionalExtension, params};

impl Database {
    /// Verify that the token exists and is staging; return false for an already-promoted token.
    /// (Xác minh token tồn tại và còn staging; trả false nếu token đã promoted.)
    pub(in crate::database::database::generation_lifecycle::promote::promote_impl) fn verify_staging_cas_token(
        &self,
        project_root: &str,
        generation: i64,
    ) -> std::result::Result<bool, CasGenerationError> {
        // Verify the token before any mutation; invalid tokens touch zero rows.
        // (Kiểm token trước mọi mutation; token không hợp lệ không đụng row nào.)
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
            Some(_) => {
                // Already-promoted is a verified idempotent no-op.
                // (Token đã promoted là no-op idempotent đã kiểm chứng.)
                Ok(false)
            }
            None => Err(CasGenerationError::UnknownToken {
                project_root: project_root.to_string(),
                generation,
            }),
        }
    }
}
