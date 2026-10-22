//! Existing Lib ecosystem marker detection.
//! Nhận diện marker ecosystem Lib hiện hành.

use std::path::Path;

use crate::language::{LibLanguage, find_csproj, is_regular_manifest};

/// Return the first language selected by the existing marker order.
/// Trả ngôn ngữ đầu tiên được chọn theo thứ tự marker hiện hành.
pub(in crate::language::detection) fn detect_marker_language(root: &Path) -> Option<LibLanguage> {
    if is_regular_manifest(&root.join("package.json")) {
        return Some(LibLanguage::Ts);
    }
    if is_regular_manifest(&root.join("Cargo.toml")) {
        return Some(LibLanguage::Rust);
    }
    if is_regular_manifest(&root.join("go.mod")) {
        return Some(LibLanguage::Go);
    }
    if is_regular_manifest(&root.join("gradle/verification-metadata.xml"))
        || is_regular_manifest(&root.join("build.gradle"))
        || is_regular_manifest(&root.join("build.gradle.kts"))
        || is_regular_manifest(&root.join("pom.xml"))
    {
        return Some(LibLanguage::Java);
    }
    if is_regular_manifest(&root.join("packages.lock.json")) || find_csproj(root).is_some() {
        return Some(LibLanguage::DotNet);
    }
    if is_regular_manifest(&root.join("pyproject.toml")) {
        return Some(LibLanguage::Python);
    }
    None
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
