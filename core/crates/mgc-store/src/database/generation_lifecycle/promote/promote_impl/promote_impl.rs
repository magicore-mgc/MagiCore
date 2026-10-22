//! Coordinate promotion phases inside the active SQLite transaction.
//! Điều phối các phase promote bên trong transaction SQLite hiện hành.
use crate::database::database::{errors::CasGenerationError, types::Database};

impl Database {
    /// Verify, retire old claims, and flip a staging token inside the caller's transaction.
    /// (Kiểm chứng, nghỉ hưu claim cũ và đổi trạng thái token trong transaction của caller.)
    pub(in crate::database::database::generation_lifecycle::promote) fn promote_cas_generation_impl(
        &self,
        project_root: &str,
        generation: i64,
    ) -> std::result::Result<(), CasGenerationError> {
        if !self.verify_staging_cas_token(project_root, generation)? {
            return Ok(());
        }

        self.retire_older_cas_generation_claims(project_root, generation)?;
        self.flip_cas_generation_to_promoted(project_root, generation)
    }
}
