//! Walks installed dependency trees and collects scripted package identities.
//! Quét cây dependency đã cài và thu thập danh tính package có lifecycle script.

use std::collections::HashSet;
use std::path::Path;

use super::super::{PendingResult, TrustPendingError};
use super::package_json::check_package_json;

pub(in crate::commands::trust::pending) fn scan_scripted_packages(
    node_modules: &Path,
) -> PendingResult<HashSet<(String, String)>> {
    let mut scripted = HashSet::new();
    let mut visited = HashSet::new();
    let mut stack = vec![node_modules.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let canonical = std::fs::canonicalize(&dir)
            .map_err(|error| TrustPendingError::scan_failed(&dir, error))?;
        if !visited.insert(canonical) {
            continue;
        }
        for entry in
            std::fs::read_dir(&dir).map_err(|error| TrustPendingError::scan_failed(&dir, error))?
        {
            let entry = entry.map_err(|error| TrustPendingError::scan_failed(&dir, error))?;
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                continue;
            }
            let path = entry.path();
            if path
                .metadata()
                .map_err(|error| TrustPendingError::scan_failed(&path, error))?
                .is_dir()
            {
                if name.starts_with('@') {
                    for sub in std::fs::read_dir(&path)
                        .map_err(|error| TrustPendingError::scan_failed(&path, error))?
                    {
                        let sub =
                            sub.map_err(|error| TrustPendingError::scan_failed(&path, error))?;
                        let sub_path = sub.path();
                        let package_name = format!("{name}/{}", sub.file_name().to_string_lossy());
                        check_package_json(&sub_path, &package_name, &mut scripted)?;
                        push_nested_node_modules(&sub_path, &mut stack)?;
                    }
                    continue;
                }
                check_package_json(&path, &name, &mut scripted)?;
                push_nested_node_modules(&path, &mut stack)?;
            }
        }
    }
    Ok(scripted)
}
fn push_nested_node_modules(
    package_dir: &Path,
    stack: &mut Vec<std::path::PathBuf>,
) -> PendingResult<()> {
    let nested = package_dir.join("node_modules");
    match nested.metadata() {
        Ok(metadata) if metadata.is_dir() => stack.push(nested),
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(TrustPendingError::scan_failed(&nested, error)),
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/package_scan.rs"]
mod tests;
