//! Walk the source tree safely, reject links/special files, and build the SHA-256 map.
//! Duyệt source tree an toàn, từ chối symlink/special file và tạo map SHA-256.

use mgc_types::{MgError, MgResult};
use std::collections::BTreeMap;
use std::path::Path;

pub(in crate::install::cargo_compat::source_checksum::implementation) fn hash_source_files(
    root: &Path,
) -> MgResult<BTreeMap<String, String>> {
    use sha2::{Digest, Sha256};

    let metadata = std::fs::symlink_metadata(root)
        .map_err(|error| MgError::Other(format!("inspect managed Cargo source: {error}")))?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(MgError::Other(
            "managed Cargo source must be a regular directory".to_string(),
        ));
    }
    let mut pending = vec![root.to_path_buf()];
    let mut files = BTreeMap::new();
    while let Some(directory) = pending.pop() {
        let mut entries = std::fs::read_dir(&directory)
            .map_err(|error| MgError::Other(format!("read managed Cargo source: {error}")))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| MgError::Other(format!("read managed Cargo source entry: {error}")))?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            if entry.file_name() == ".cargo-checksum.json" {
                continue;
            }
            let path = entry.path();
            let metadata = std::fs::symlink_metadata(&path)
                .map_err(|error| MgError::Other(format!("inspect managed Cargo file: {error}")))?;
            if metadata.file_type().is_symlink() {
                return Err(MgError::Other(format!(
                    "managed Cargo source contains a symbolic link: {}",
                    path.display()
                )));
            }
            if metadata.is_dir() {
                pending.push(path);
                continue;
            }
            if !metadata.is_file() {
                return Err(MgError::Other(format!(
                    "managed Cargo source contains a special file: {}",
                    path.display()
                )));
            }
            let relative = path.strip_prefix(root).map_err(|error| {
                MgError::Other(format!(
                    "managed Cargo path escapes source directory: {error}"
                ))
            })?;
            let mut parts = Vec::new();
            for component in relative.components() {
                let std::path::Component::Normal(part) = component else {
                    return Err(MgError::Other(
                        "managed Cargo source path has an unsafe component".to_string(),
                    ));
                };
                parts.push(part.to_str().ok_or_else(|| {
                    MgError::Other("managed Cargo source path is not UTF-8".to_string())
                })?);
            }
            let contents = std::fs::read(&path)
                .map_err(|error| MgError::Other(format!("read managed Cargo file: {error}")))?;
            files.insert(parts.join("/"), hex::encode(Sha256::digest(contents)));
        }
    }
    Ok(files)
}
