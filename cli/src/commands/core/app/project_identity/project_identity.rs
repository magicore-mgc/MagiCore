//! App project identity and language selection.
//! Nhận diện project App và chọn ngôn ngữ.

use anyhow::Result;
use std::path::{Path, PathBuf};

/// Resolve and validate the current App project root.
/// Tìm và xác thực root project App hiện tại.
pub fn project_root() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    mgc_app_adapter::adapter_for(&cwd).ok_or_else(|| {
        anyhow::Error::new(super::super::error::AppCoreError::project_not_detected())
    })?;
    crate::commands::core::shared::ensure_project_core_identity(&cwd, "app")?;
    Ok(cwd)
}

/// Detect the App language from the project manifests.
/// Nhận diện ngôn ngữ App qua manifest của project.
pub fn language(root: &Path) -> Result<mgc_app_adapter::AppLanguage> {
    mgc_app_adapter::adapter_for(root)
        .map(|adapter| adapter.language)
        .ok_or_else(|| {
            anyhow::Error::new(super::super::error::AppCoreError::language_not_detected(
                root,
            ))
        })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
