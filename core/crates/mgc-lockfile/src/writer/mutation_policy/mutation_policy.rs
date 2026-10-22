//! Protects schema-specific lock writes and legacy signing routes.
//! Bảo vệ ghi lock theo schema và route ký legacy.

use crate::{LockfileError, LockfileResult};
use std::path::Path;

use super::error::MutationPolicyError;

/// Refuse ordinary mutation while signature evidence or signed metadata exists.
/// Chặn mutation thường khi còn artifact chữ ký hoặc metadata ký.
pub fn ensure_lockfile_mutation_allowed(path: &Path) -> LockfileResult<()> {
    let signature_path = crate::parser::signature_path_for(path);
    match std::fs::symlink_metadata(&signature_path) {
        Ok(_) => {
            return Err(MutationPolicyError::SignedArtifact {
                path: signature_path.display().to_string(),
            }
            .into());
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(source) => {
            return Err(MutationPolicyError::SignatureInspection {
                path: signature_path.display().to_string(),
                source,
            }
            .into());
        }
    }

    match std::fs::symlink_metadata(path) {
        Ok(metadata) if !metadata.file_type().is_file() => {
            return Err(MutationPolicyError::NonRegularPath {
                path: path.display().to_string(),
            }
            .into());
        }
        Ok(_) => {
            let bytes = crate::parser::read_lockfile_bytes(path)?;
            let text = std::str::from_utf8(&bytes).map_err(|error| {
                LockfileError::ParseError(format!("lockfile is not UTF-8: {error}"))
            })?;
            let document: toml::Value = toml::from_str(text)?;
            let metadata = document
                .get("metadata")
                .and_then(toml::Value::as_table)
                .ok_or_else(|| MutationPolicyError::MissingMetadata {
                    path: path.display().to_string(),
                })?;
            if metadata.contains_key("signer") || metadata.contains_key("signature") {
                return Err(MutationPolicyError::SignedMetadata {
                    path: path.display().to_string(),
                }
                .into());
            }
            if document
                .get("version")
                .and_then(toml::Value::as_str)
                .is_some_and(crate::v4::is_v4_schema_version)
            {
                return Err(MutationPolicyError::LegacyV4Path {
                    path: path.display().to_string(),
                }
                .into());
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(source) => {
            return Err(MutationPolicyError::LockInspection {
                path: path.display().to_string(),
                source,
            }
            .into());
        }
    }

    Ok(())
}

/// Refuse routing a v4 document through the legacy signing model.
/// Chặn route document v4 qua model ký legacy.
pub(in crate::writer) fn ensure_not_v4_legacy_write(path: &Path) -> LockfileResult<()> {
    let bytes = match crate::parser::read_lockfile_bytes(path) {
        Ok(bytes) => bytes,
        Err(LockfileError::IoError(error)) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(());
        }
        Err(error) => return Err(error),
    };
    let text = std::str::from_utf8(&bytes)
        .map_err(|error| LockfileError::ParseError(format!("lockfile is not UTF-8: {error}")))?;
    if crate::detect_lockfile_version(text)? == 4 {
        return Err(MutationPolicyError::LegacyV4Path {
            path: path.display().to_string(),
        }
        .into());
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
