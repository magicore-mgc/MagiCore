//! Owns PEP 508 name projection and Python freshness. — Sở hữu projection tên PEP 508 và freshness Python.

use std::path::Path;

use anyhow::Result;

use super::super::manifest_file::check_manifest_file;

/// PEP 508 requirement to distribution name (`Foo_Bar>=1.0; marker` → `foo-bar`).
/// Requirement PEP 508 thành tên distribution.
fn pep508_name(requirement: &str) -> String {
    let bare = requirement.split(';').next().unwrap_or(requirement).trim();
    let end = bare
        .find(|c: char| " \t<>=!~([".contains(c))
        .unwrap_or(bare.len());
    bare[..end].trim().replace('_', "-").to_ascii_lowercase()
}
/// Declared Python runtime and MGC-managed dev dependencies from `pyproject.toml`.
/// Dependency runtime và nhóm dev do MGC quản lý trong `pyproject.toml`.
pub fn declared_python_deps(pyproject_toml: &str) -> Vec<String> {
    let Ok(table) = pyproject_toml.parse::<toml::Table>() else {
        return Vec::new();
    };
    let mut names = Vec::new();
    // Unselected optional extras are not roots in the default install graph.
    // Extra chưa được chọn không thuộc root graph của install mặc định.
    if let Some(deps) = table
        .get("project")
        .and_then(|v| v.get("dependencies"))
        .and_then(|v| v.as_array())
    {
        names.extend(
            deps.iter()
                .filter_map(|v| v.as_str())
                .map(pep508_name)
                .filter(|n| !n.is_empty()),
        );
    }
    if let Some(deps) = table
        .get("dependency-groups")
        .and_then(|v| v.get("dev"))
        .and_then(|v| v.as_array())
    {
        names.extend(
            deps.iter()
                .filter_map(|v| v.as_str())
                .map(pep508_name)
                .filter(|n| !n.is_empty()),
        );
    }
    names.sort();
    names.dedup();
    names
}
/// Python freshness for AI/dev flows (`pyproject.toml`).
/// Freshness Python cho luồng AI/dev (`pyproject.toml`).
pub fn check_python_freshness(project_root: &Path) -> Result<()> {
    check_manifest_file(
        project_root,
        "pyproject.toml",
        true,
        "ai",
        mgc_lockfile::EcosystemTag::Python,
        |content| Ok(declared_python_deps(content)),
    )?;
    Ok(())
}
