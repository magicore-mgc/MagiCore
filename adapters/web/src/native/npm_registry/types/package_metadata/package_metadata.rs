//! types package metadata.
// Chứa đúng trách nhiệm của module; giữ nguyên hành vi, API, chính sách lỗi và bảo mật hiện có.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub description: Option<String>,
    pub versions: std::collections::HashMap<String, VersionInfo>,
    #[serde(rename = "dist-tags")]
    pub dist_tags: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub time: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub version: String,
    pub dependencies: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "peerDependencies")]
    pub peer_dependencies: Option<std::collections::HashMap<String, String>>,
    #[serde(default, rename = "peerDependenciesMeta")]
    pub peer_dependencies_meta: std::collections::HashMap<String, PeerDependencyMeta>,
    #[serde(rename = "optionalDependencies")]
    pub optional_dependencies: Option<std::collections::HashMap<String, String>>,
    pub os: Option<Vec<String>>,
    pub cpu: Option<Vec<String>>,
    pub dist: Option<DistInfo>,
}

/// npm marks peer edges that must not be auto-installed via this metadata.
/// (npm dùng metadata này để đánh dấu peer không được tự cài.)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PeerDependencyMeta {
    #[serde(default)]
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistInfo {
    pub tarball: String,
    #[serde(rename = "integrity")]
    pub integrity: Option<String>,
}
