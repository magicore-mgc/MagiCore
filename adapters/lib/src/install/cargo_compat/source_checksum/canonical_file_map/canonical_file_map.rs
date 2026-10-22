//! Match canonical paths when a filesystem folds case aliases.
//! So khớp đường dẫn canonical khi filesystem gộp alias hoa-thường.

use mgc_types::{MgError, MgResult};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Compare extracted files using filesystem identity for archive case aliases.
/// So sánh file đã giải nén theo identity filesystem khi archive có tên khác hoa/thường.
///
/// Some published crates contain identical files whose names differ only by
/// case. Case-insensitive filesystems materialize them as one physical file;
/// canonical path comparison accepts only identical bytes at that same file.
/// (Một số crate có file trùng nội dung nhưng khác hoa/thường; chỉ gộp khi
/// filesystem thật sự ánh xạ chúng vào cùng file vật lý và hash trùng nhau.)
pub(in crate::install::cargo_compat::source_checksum::implementation) fn source_files_match_archive(
    package_root: &Path,
    expected: &BTreeMap<String, String>,
    actual: &BTreeMap<String, String>,
) -> MgResult<bool> {
    if expected == actual {
        return Ok(true);
    }
    let canonical_root = std::fs::canonicalize(package_root)
        .map_err(|error| MgError::Other(format!("canonicalize Cargo source root: {error}")))?;
    let expected = canonical_source_file_map(package_root, &canonical_root, expected)?;
    let Some(expected) = expected else {
        return Ok(false);
    };
    let actual = canonical_source_file_map(package_root, &canonical_root, actual)?;
    Ok(actual.is_some_and(|actual| actual == expected))
}

pub(in crate::install::cargo_compat::source_checksum::implementation) fn canonical_source_file_map(
    package_root: &Path,
    canonical_root: &Path,
    files: &BTreeMap<String, String>,
) -> MgResult<Option<BTreeMap<PathBuf, String>>> {
    let mut canonical_files = BTreeMap::new();
    for (relative, digest) in files {
        let relative_path = Path::new(relative);
        if relative_path.as_os_str().is_empty()
            || relative_path
                .components()
                .any(|component| !matches!(component, std::path::Component::Normal(_)))
        {
            return Ok(None);
        }
        let path = match std::fs::canonicalize(package_root.join(relative_path)) {
            Ok(path) => path,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => {
                return Err(MgError::Other(format!(
                    "canonicalize Cargo source file '{relative}': {error}"
                )));
            }
        };
        let Ok(relative_path) = path.strip_prefix(canonical_root) else {
            return Ok(None);
        };
        match canonical_files.get(relative_path) {
            Some(existing) if existing != digest => return Ok(None),
            Some(_) => {}
            None => {
                canonical_files.insert(relative_path.to_path_buf(), digest.clone());
            }
        }
    }
    Ok(Some(canonical_files))
}
