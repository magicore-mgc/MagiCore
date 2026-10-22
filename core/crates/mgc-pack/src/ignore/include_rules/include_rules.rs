//! Forced package file policy — luật file package bắt buộc.
use std::path::Path;

/// Required package metadata files — file metadata bắt buộc của package.
pub(in crate::ignore) const ALWAYS_INCLUDE: &[&str] =
    &["package.json", "README.md", "LICENSE", "CHANGELOG.md"];

/// Never package generated or repository metadata — không đóng gói dependency hoặc metadata repository.
const ALWAYS_EXCLUDE: &[&str] = &["node_modules", ".git", ".magicore"];

/// Decide whether a relative path survives forced exclusions.
/// Xác định path tương đối có vượt qua luật loại trừ cứng hay không.
pub(in crate::ignore) fn should_include(rel: &Path) -> bool {
    let components = rel
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    if components
        .iter()
        .any(|c| ALWAYS_EXCLUDE.contains(&c.as_str()))
    {
        return false;
    }
    true
}
