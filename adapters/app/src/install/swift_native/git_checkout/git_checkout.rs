//! Guard Git checkout materialization through the Swift protocol only.
//! Chặn materialize Git checkout qua đúng Swift protocol.

use mgc_resolver::protocols::{ResolvedEntry, SwiftRegistryProtocol};
use mgc_types::MgResult;
use std::path::Path;

/// Keep the resolver's Unsupported result; never invoke a system Git executable.
/// Giữ kết quả Unsupported của resolver; tuyệt đối không gọi Git executable hệ thống.
pub(in crate::install) fn materialize_git_checkout(
    protocol: &SwiftRegistryProtocol,
    entry: &ResolvedEntry,
    expected_commit: Option<&str>,
    swift_root: &Path,
) -> MgResult<()> {
    protocol.materialize_git(entry, expected_commit, swift_root)?;
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
