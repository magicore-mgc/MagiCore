use super::super::types::Database;
use super::CasGenerationGuard;
use anyhow::Result;
use std::path::Path;

impl Database {
    /// Begin a staging generation and return it guarded by RAII: any drop
    /// without `disarm()` aborts the token (Gate 11-A item 6). The guard
    /// needs the DB PATH (not a borrow) so it can survive across awaits
    /// in async callers.
    /// (Begin staging generation và trả kèm guard RAII: drop nào không
    /// `disarm()` sẽ hủy token (Gate 11-A mục 6). Guard cần PATH DB
    /// (không phải borrow) để sống qua các await trong caller async.)
    pub fn begin_cas_generation_guarded(
        &self,
        db_path: &Path,
        project_root: &str,
    ) -> Result<CasGenerationGuard> {
        let generation = self.begin_cas_generation(project_root)?;
        Ok(CasGenerationGuard {
            db_path: db_path.to_path_buf(),
            project_root: project_root.to_string(),
            generation,
            armed: true,
        })
    }
}
