//! Exact package-record merge — does not conflate ownership or graph metadata.
//! Gộp record package chính xác — không trộn owner hay metadata graph.
use super::error::LockfileMergeError;

/// Merge duplicate lockfile entries into a merged Lockfile (no-op if none).
/// Gộp entry lockfile trùng vào Lockfile mới (không làm gì nếu không có).
pub(in crate::commands::dedupe) fn merged_lockfile(
    lock: &mgc_lockfile::Lockfile,
) -> std::result::Result<(mgc_lockfile::Lockfile, usize), LockfileMergeError> {
    // Package name/version alone is not a unified-lock identity: identical
    // names can belong to different cores, ecosystems, registries, sources,
    // or carry distinct graph/target metadata. Deduplicate only exact full
    // serialized records so this GC-oriented command cannot erase ownership.
    // (Chỉ khử bản ghi đầy đủ giống hệt để không làm mất owner/graph/target.)
    let mut seen = std::collections::HashSet::new();
    let mut merged = 0usize;
    let mut new_packages = Vec::new();
    for pkg in &lock.packages {
        let key = toml::to_string(pkg).map_err(LockfileMergeError::SerializePackageIdentity)?;
        if seen.insert(key) {
            new_packages.push(pkg.clone());
        } else {
            merged += 1;
        }
    }
    let mut new_lock = lock.clone();
    new_lock.packages = new_packages;
    Ok((new_lock, merged))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
