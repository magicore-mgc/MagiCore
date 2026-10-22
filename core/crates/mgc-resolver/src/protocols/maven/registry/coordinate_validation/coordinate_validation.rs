//! Parse Maven coordinates and guard their cache path components.
//! Parse tọa độ Maven và chặn component thoát khỏi cache.
use crate::protocols::maven::registry::registry::MavenProtocol;
use mgc_types::{MgError, MgResult};

impl MavenProtocol {
    /// Split a `groupId:artifactId` coordinate.
    /// Tách tọa độ `groupId:artifactId`.
    pub fn split_coordinate(name: &str) -> MgResult<(String, String)> {
        let (group, artifact) = name.split_once(':').ok_or_else(|| {
            MgError::Other(format!(
                "maven coordinate must be 'group:artifact': '{name}'"
            ))
        })?;
        if group.is_empty() || artifact.is_empty() {
            return Err(MgError::Other(format!(
                "maven coordinate has an empty part: '{name}'"
            )));
        }
        // Each dot-separated group segment becomes a cache directory.
        // Mỗi phần group thành thư mục; kiểm tra trước mọi thao tác ghi cache.
        for segment in group.split('.') {
            validate_maven_cache_component(segment, "group")?;
        }
        validate_maven_cache_component(artifact, "artifact")?;
        Ok((group.to_string(), artifact.to_string()))
    }
}

/// Reject coordinates that can change the cache path's directory structure.
/// Chặn component thay đổi cấu trúc đường dẫn, giữ nguyên version Maven thô.
pub(in crate::protocols::maven::registry) fn validate_maven_cache_component(
    value: &str,
    kind: &str,
) -> MgResult<()> {
    if value.is_empty()
        || matches!(value, "." | "..")
        || value.trim() != value
        || value.ends_with('.')
        || value
            .chars()
            .any(|c| c.is_control() || matches!(c, '/' | '\\' | ':'))
    {
        return Err(MgError::Other(format!(
            "unsafe Maven cache {kind} component"
        )));
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
