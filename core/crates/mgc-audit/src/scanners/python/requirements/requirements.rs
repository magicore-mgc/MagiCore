use mgc_types::{MgError, MgResult};
use std::path::Path;

use super::super::normalize_python_name;
use crate::scanners::{osv, read_bounded_project_input};

/// Read exact pins from requirements files; coverage is always Partial.
/// Đọc pin chính xác từ requirements; độ phủ luôn là Partial.
pub fn read_requirements_pins(project_root: &Path) -> MgResult<(Vec<osv::OsvPin>, Vec<String>)> {
    let mut paths: Vec<_> = std::fs::read_dir(project_root)
        .map_err(|error| MgError::Other(format!("read project directory: {error}")))?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| {
            path.is_file()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with("requirements") && name.ends_with(".txt"))
        })
        .collect();
    paths.sort();
    let mut pins = std::collections::BTreeSet::new();
    let mut skipped =
        vec!["requirements files do not prove a complete resolved dependency graph".to_string()];
    for path in paths {
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("requirements.txt");
        let raw = read_bounded_project_input(&path, file_name)?;
        for (index, line) in raw.lines().enumerate() {
            let line = line.split('#').next().unwrap_or("").trim();
            if line.is_empty() || line.starts_with("--hash=") {
                continue;
            }
            if line.starts_with('-') {
                skipped.push(format!(
                    "{file_name}:{} uses an unsupported include/option",
                    index + 1
                ));
                continue;
            }
            let requirement = line
                .split(';')
                .next()
                .unwrap_or("")
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim();
            let name_end = requirement
                .find(|ch: char| !ch.is_ascii_alphanumeric() && !"._-[]".contains(ch))
                .unwrap_or(requirement.len());
            let raw_name = &requirement[..name_end];
            let package_name = raw_name.split('[').next().unwrap_or("").trim();
            let constraint = requirement[name_end..].trim();
            let version = constraint
                .strip_prefix("===")
                .or_else(|| constraint.strip_prefix("=="))
                .map(str::trim);
            match (package_name.is_empty(), version) {
                (false, Some(version)) if !version.is_empty() && !version.contains('*') => {
                    pins.insert((normalize_python_name(package_name), version.to_string()));
                    if line.contains(';') {
                        skipped.push(format!(
                            "{file_name}:{} has an environment marker; the query includes it conservatively",
                            index + 1
                        ));
                    }
                }
                _ => skipped.push(format!(
                    "{file_name}:{} is not an exact package version pin",
                    index + 1
                )),
            }
        }
    }
    Ok((
        pins.into_iter()
            .map(|(name, version)| osv::OsvPin {
                name,
                version,
                ecosystem: "PyPI",
            })
            .collect(),
        skipped,
    ))
}
