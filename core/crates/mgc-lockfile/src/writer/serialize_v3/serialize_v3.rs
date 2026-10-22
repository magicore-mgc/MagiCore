//! Serializes v3 lock data and rejects incomplete classified package sources.
//! Serialize lock v3 và chặn nguồn package phân loại chưa đầy đủ.

use crate::{Lockfile, LockfileResult};
use std::path::Path;

/// Serialize a lockfile after validating classified package source ownership.
/// Serialize lockfile sau khi xác minh nguồn của package đã phân loại.
pub fn serialize_lockfile(lockfile: &Lockfile) -> LockfileResult<String> {
    crate::verifier::verify_ownership_completeness(lockfile)?;
    let toml_str = toml::to_string_pretty(lockfile)?;
    Ok(toml_str)
}

/// Write a lockfile to a path after checking mutation policy.
/// Ghi lockfile ra path sau khi kiểm tra policy mutation.
pub fn write_lockfile(lockfile: &Lockfile, path: &Path) -> LockfileResult<()> {
    crate::writer::ensure_lockfile_mutation_allowed(path)?;
    write_lockfile_unchecked(lockfile, path)
}

/// Serialize and write a lockfile for the dedicated signing flow.
/// Serialize và ghi lockfile cho luồng ký chuyên dụng.
pub(in crate::writer) fn write_lockfile_unchecked(
    lockfile: &Lockfile,
    path: &Path,
) -> LockfileResult<()> {
    let toml_str = serialize_lockfile(lockfile)?;
    std::fs::write(path, toml_str)?;
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
