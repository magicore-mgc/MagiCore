//! Deterministic root-level csproj selection.
//! Chọn csproj root-level theo cách xác định.

use std::path::{Path, PathBuf};

use crate::language::is_regular_manifest;

/// Find one regular csproj at the project root; multiple projects are ambiguous.
/// Tìm csproj thường duy nhất ở root; nhiều project là ambiguous.
pub(crate) fn find_csproj(root: &Path) -> Option<PathBuf> {
    let entries = std::fs::read_dir(root).ok()?;
    let mut projects = entries.flatten().map(|entry| entry.path()).filter(|path| {
        is_regular_manifest(path)
            && path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("csproj"))
    });
    let first = projects.next()?;
    projects.next().is_none().then_some(first)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
