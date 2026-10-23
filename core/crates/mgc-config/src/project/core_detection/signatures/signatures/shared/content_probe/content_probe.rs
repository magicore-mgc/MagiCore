//! Manifest content probes for core signature detection.
//! Probe nội dung manifest cho nhận diện signature core.

use std::path::{Path, PathBuf};

/// Match known framework coordinates without interpreting version syntax.
/// Nhận diện tọa độ framework phổ biến, không phân tích constraint phiên bản.
pub(in crate::project::project::core_detection::signatures::signatures) fn text_contains_any(
    content: &str,
    markers: &[&str],
) -> bool {
    let lower = content.to_ascii_lowercase();
    markers
        .iter()
        .any(|marker| lower.contains(&marker.to_ascii_lowercase()))
}

/// Check dependency tables, including Cargo workspace and target-specific tables.
/// Kiểm tra dependency table kể cả workspace và cấu hình theo target.
pub(in crate::project::project::core_detection::signatures::signatures) fn has_named_dependency(
    manifest: &toml::Value,
    names: &[&str],
) -> bool {
    fn in_table(value: &toml::Value, names: &[&str]) -> bool {
        let Some(table) = value.as_table() else {
            return false;
        };
        let has_dependency = ["dependencies", "dev-dependencies", "build-dependencies"]
            .iter()
            .filter_map(|section| table.get(*section).and_then(toml::Value::as_table))
            .any(|dependencies| {
                dependencies.iter().any(|(name, spec)| {
                    names.contains(&name.as_str())
                        || spec
                            .get("package")
                            .and_then(toml::Value::as_str)
                            .is_some_and(|package| names.contains(&package))
                })
            });
        has_dependency
            || ["workspace", "target"]
                .iter()
                .filter_map(|section| table.get(*section))
                .any(|nested| in_table(nested, names))
    }

    in_table(manifest, names)
}

/// Find a regular root-level manifest by extension, rejecting a matching symlink.
/// Tìm manifest thường theo extension và từ chối file khớp là symlink.
pub(in crate::project::project::core_detection::signatures::signatures) fn has_regular_extension_file(
    root: &Path,
    extension: &str,
) -> anyhow::Result<bool> {
    Ok(!files_with_extension(root, extension)?.is_empty())
}

/// Collect regular root-level manifests by extension, rejecting matching symlinks.
/// Thu thập manifest thường theo extension và từ chối file khớp là symlink.
pub(in crate::project::project::core_detection::signatures::signatures) fn files_with_extension(
    root: &Path,
    extension: &str,
) -> anyhow::Result<Vec<PathBuf>> {
    let entries = match std::fs::read_dir(root) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error.into()),
    };
    let mut files = Vec::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if !path
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|value| value.eq_ignore_ascii_case(extension))
        {
            continue;
        }
        let file_type = entry.file_type()?;
        if file_type.is_symlink() {
            anyhow::bail!(
                "project signature '{}' must not be a symlink",
                path.display()
            );
        }
        if file_type.is_file() {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}
