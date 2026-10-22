//! Owns Cargo dependency projection and Rust/library freshness gates. — Sở hữu projection dependency Cargo và freshness gate Rust/library.

use std::path::Path;

use anyhow::Result;

use super::super::manifest_file::check_manifest_file;

/// Declared Rust dependencies from `Cargo.toml` tables.
/// Dependency Rust khai báo từ các bảng `Cargo.toml`.
pub fn declared_cargo_deps(cargo_toml: &str) -> Vec<String> {
    let Ok(table) = cargo_toml.parse::<toml::Table>() else {
        return Vec::new();
    };
    let mut names = Vec::new();
    for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
        if let Some(deps) = table.get(section).and_then(|v| v.as_table()) {
            names.extend(deps.keys().cloned());
        }
    }
    // Target-specific tables (`[target.'cfg(...)'.dependencies]`).
    // (Bảng theo target (`[target.'cfg(...)'.dependencies]`).)
    if let Some(target) = table.get("target").and_then(|v| v.as_table()) {
        for triple in target.values().filter_map(|v| v.as_table()) {
            for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
                if let Some(deps) = triple.get(section).and_then(|v| v.as_table()) {
                    names.extend(deps.keys().cloned());
                }
            }
        }
    }
    names.sort();
    names.dedup();
    names
}
/// Cargo freshness for Rust dev flows (`Cargo.toml`, e.g. Bevy/ESP32).
/// Freshness Cargo cho luồng dev Rust (`Cargo.toml`, vd Bevy/ESP32).
pub fn check_cargo_freshness(project_root: &Path) -> Result<()> {
    check_manifest_file(
        project_root,
        "Cargo.toml",
        true,
        "game",
        mgc_lockfile::EcosystemTag::Rust,
        |content| Ok(declared_cargo_deps(content)),
    )?;
    Ok(())
}
/// Cargo freshness for Rust projects owned by the library core.
/// Freshness Cargo cho project Rust thuộc core library.
pub fn check_lib_cargo_freshness(project_root: &Path) -> Result<()> {
    check_manifest_file(
        project_root,
        "Cargo.toml",
        true,
        "lib",
        mgc_lockfile::EcosystemTag::Rust,
        |content| Ok(declared_cargo_deps(content)),
    )?;
    Ok(())
}
