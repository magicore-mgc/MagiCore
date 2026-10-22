//! Owns manifest reads, required-lock behavior and callback-based projection checking. — Sở hữu đọc manifest, quy tắc bắt buộc lock và kiểm tra projection qua callback.

use std::path::Path;

use anyhow::{Context, Result};

use super::super::lock::check_manifest_roots;
use super::super::manifest_syntax::validate_manifest_syntax;

/// Read a manifest, project its dependency names, and check the lock.
/// Returns `Ok(false)` when the manifest is absent (nothing to compare).
/// Đọc manifest, chiếu tên dependency và đối chiếu lock. Trả `Ok(false)`
/// khi manifest vắng mặt (không có gì để so sánh).
pub(in crate::commands::freshness) fn check_manifest_file(
    project_root: &Path,
    file: &str,
    require_lock: bool,
    owner_fallback: &str,
    ecosystem: mgc_lockfile::EcosystemTag,
    project: impl FnOnce(&str) -> Result<Vec<String>>,
) -> Result<bool> {
    let path = project_root.join(file);
    if !path.exists() {
        return Ok(false);
    }
    let has_lock = project_root.join("mgc.lock").exists();
    if require_lock && !has_lock {
        return Err(crate::error::stale_lock_needs_install(
            "mgc.lock is missing",
        ));
    }
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("cannot read {}", path.display()))?;
    validate_manifest_syntax(file, &content)?;
    let declared = project(&content)?;
    if has_lock {
        check_manifest_roots(project_root, owner_fallback, ecosystem, &declared)?;
    }
    Ok(true)
}
