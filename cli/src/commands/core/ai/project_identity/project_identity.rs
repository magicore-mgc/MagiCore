//! AI project-root selection and identity validation.
//! Chọn root project AI và xác thực identity.

#[cfg(feature = "ai")]
use anyhow::Result;
#[cfg(feature = "ai")]
use std::path::{Path, PathBuf};

/// Resolve and validate the current AI project root.
/// Tìm và xác thực root project AI hiện tại.
#[cfg(feature = "ai")]
pub fn ai_project_root() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    ai_project_root_at(&cwd)
}

/// Validate one AI root without changing process-global current directory.
/// Xác thực root AI mà không đổi working directory toàn process.
#[cfg(feature = "ai")]
fn ai_project_root_at(root: &Path) -> Result<PathBuf> {
    mgc_ai_adapter::adapter_for(root).ok_or_else(|| {
        anyhow::Error::new(super::super::error::AiCoreError::project_not_detected())
    })?;
    crate::commands::core::shared::ensure_project_core_identity(root, "ai")?;
    Ok(root.to_path_buf())
}

#[cfg(all(test, feature = "ai"))]
#[path = "test/mod.rs"]
mod tests;
