//! Shared Web scaffold conventions: package-name normalization and baseline files.
//! Quy ước scaffold Web dùng chung: chuẩn hoá tên package và file nền.

use std::path::Path;

use anyhow::Result;

use super::super::super::write_file;

/// Normalize a project name into a lowercase, hyphen-separated package name.
pub(crate) fn package_name(name: &str) -> String {
    let mut result = String::new();
    for character in name.chars() {
        if character.is_ascii_alphanumeric() {
            result.push(character.to_ascii_lowercase());
        } else if !result.is_empty() && !result.ends_with('-') {
            result.push('-');
        }
    }
    let result = result.trim_matches('-').to_string();
    if result.is_empty() {
        "mgc-web-app".to_string()
    } else {
        result
    }
}

/// Baseline ignore file and README shared by every native Web project.
pub(crate) fn write_baseline_files(target: &Path, name: &str, framework: &str) -> Result<()> {
    write_file(
        &target.join(".gitignore"),
        "/node_modules/\n/target/\n/dist/\n.env\n",
    )?;
    write_file(
        &target.join("README.md"),
        &format!("# {name}\n\n{framework} project scaffolded by MagiCore.\n"),
    )
}
