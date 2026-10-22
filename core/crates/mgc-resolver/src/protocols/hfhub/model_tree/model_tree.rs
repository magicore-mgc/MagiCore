//! Parse and validate model tree file entries.
//! Parse và kiểm tra các entry file của model tree.

use serde_json::Value;

use super::super::model_file::ModelFile;
use super::error::ModelTreeError;
use mgc_types::{MgError, MgResult};

/// Convert registry tree entries to sorted locked files.
/// Chuyển các entry tree của registry thành file đã lock và sắp xếp.
pub(in crate::protocols::hfhub) fn parse_model_tree(
    entries: &[Value],
    model: &str,
    revision: &str,
    base_url: &str,
) -> MgResult<Vec<ModelFile>> {
    let mut files = Vec::new();
    for entry in entries {
        if entry.get("type").and_then(Value::as_str) != Some("file") {
            continue;
        }
        let path = entry
            .get("path")
            .and_then(Value::as_str)
            .ok_or_else(|| MgError::from(ModelTreeError::MissingPath))?;
        if path.contains("..") {
            return Err(ModelTreeError::EscapesRepo(path.to_string()).into());
        }
        let git_blob_sha1 = entry
            .get("oid")
            .and_then(Value::as_str)
            .filter(|oid| oid.len() == 40 && oid.chars().all(|c| c.is_ascii_hexdigit()))
            .ok_or_else(|| MgError::from(ModelTreeError::MissingBlobOid(path.to_string())))?
            .to_string();
        let (sha256, size) = match entry.get("lfs") {
            Some(lfs) => {
                let oid = lfs.get("oid").and_then(Value::as_str).ok_or_else(|| {
                    MgError::from(ModelTreeError::MissingLfsOid(path.to_string()))
                })?;
                if oid.len() != 64 || !oid.chars().all(|c| c.is_ascii_hexdigit()) {
                    return Err(ModelTreeError::InvalidLfsSha256(path.to_string()).into());
                }
                (
                    Some(oid.to_string()),
                    lfs.get("size").and_then(Value::as_u64),
                )
            }
            None => (None, entry.get("size").and_then(Value::as_u64)),
        };
        files.push(ModelFile {
            url: format!("{base_url}/{model}/resolve/{revision}/{path}"),
            path: path.to_string(),
            size,
            sha256,
            git_blob_sha1,
        });
    }
    files.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(files)
}
