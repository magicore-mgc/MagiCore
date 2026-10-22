//! Loads machine-local and committed trust policy inputs.
//! Đọc policy trust ở máy local và policy được commit trong project.

use anyhow::{Context, Result};
use mgc_store::Database;
use std::collections::HashMap;

use super::super::{PendingResult, TrustPendingError};

pub(in crate::commands::trust::pending) fn load_db_policies(
    db_path: &std::path::Path,
) -> Result<HashMap<String, String>> {
    let db = Database::open(db_path)
        .context(crate::error::trust_policy_database_open_failed(db_path))?;
    let rows = db
        .list_trust_policies()
        .context(crate::error::trust_policy_database_read_failed(db_path))?;
    Ok(rows
        .into_iter()
        .map(|(id, policy, _)| (id, policy))
        .collect())
}
pub(in crate::commands::trust::pending) fn load_file_policy(
    project_root: &std::path::Path,
) -> PendingResult<Option<mgc_config::project::ScriptsPolicy>> {
    mgc_config::project::load_scripts_table_typed(project_root)
        .map_err(TrustPendingError::scripts_policy_failed)
}

#[cfg(test)]
#[path = "test/policy.rs"]
mod tests;
