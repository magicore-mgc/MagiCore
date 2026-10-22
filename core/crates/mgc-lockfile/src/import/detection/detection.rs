//! Detect supported legacy files without importing — nhận diện file legacy mà không import.

use std::path::Path;

use super::super::{ALL, LegacyLockfile};

/// Detect supported legacy lockfiles without importing them.
/// Migration must be explicit; install callers should use this only for hints.
/// Chỉ nhận diện file; migration phải được gọi tường minh và install chỉ dùng để gợi ý.
pub fn detect_legacy_lockfiles(project_root: &Path) -> Vec<LegacyLockfile> {
    ALL.iter()
        .filter_map(|name| {
            let path = project_root.join(name);
            path.exists().then_some(LegacyLockfile {
                file_name: name,
                path,
            })
        })
        .collect()
}

/// Phát hiện nguy cơ Trust-Downgrade: đã có `mgc.lock` nhưng lại xuất hiện
/// lockfile PM cũ có nguy cơ ghi đè/nhầm lẫn.
// (Trust-downgrade risk: a signed mgc.lock exists alongside stale legacy lockfiles.)
pub fn check_trust_downgrade_risk(project_root: &Path) -> Option<Vec<&'static str>> {
    let mgc_lock = project_root.join("mgc.lock");
    if !mgc_lock.exists() {
        return None;
    }
    let legacy = detect_legacy_lockfiles(project_root);
    if legacy.is_empty() {
        None
    } else {
        Some(legacy.iter().map(|l| l.file_name).collect())
    }
}
