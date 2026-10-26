//! Build verification is root-bound — refuses to certify another project.
//! Xác minh build gắn với root — từ chối xác nhận project khác.
use mgc_config::project::ProjectConfig;
use std::path::{Path, PathBuf};

use super::error::DedupeBuildVerificationError;

/// Runtime verification (user decision 2026-08-05): build the project after
/// merging; rollback the lockfile if the build fails.
/// Xác minh build sau khi merge; rollback lockfile nếu build thất bại.
pub(in crate::commands::dedupe) async fn verify_with_build(
    project_root: &Path,
    dry_run: bool,
) -> std::result::Result<(), DedupeBuildVerificationError> {
    if dry_run {
        return Ok(());
    }
    // Root-bound verify (P1-3): build::run resolves its project from the
    // ambient cwd — verifying a DIFFERENT scope would certify the wrong
    // tree. Fail closed instead of verifying blindly (canonicalized on
    // both sides: /tmp vs /private/tmp must not false-mismatch).
    // (Verify đúng scope — cwd khác thì lỗi chứ không verify mù.)
    let cwd = std::env::current_dir()?;
    let cwd_root = ProjectConfig::find_project_root(&cwd).and_then(|root| root.canonicalize().ok());
    let want_root = project_root.canonicalize().ok();
    if !same_project_root(cwd_root, want_root) {
        return Err(DedupeBuildVerificationError::WrongProject {
            cwd: cwd.display().to_string(),
            project_root: project_root.display().to_string(),
        });
    }
    crate::commands::build::run(None, None, None, None, None, false)
        .await
        .map_err(DedupeBuildVerificationError::BuildFailed)
}

fn same_project_root(cwd_root: Option<PathBuf>, project_root: Option<PathBuf>) -> bool {
    matches!((cwd_root, project_root), (Some(cwd), Some(root)) if cwd == root)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
