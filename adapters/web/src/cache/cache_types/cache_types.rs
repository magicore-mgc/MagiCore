//! Data records persisted by the Web shared cache.
//! Record dữ liệu được lưu bởi cache dùng chung của Web.

use mgc_types::adapter::ResolvedGraph;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedResolutionEnvelope {
    pub cache_version: u32,
    pub registry_url: String,
    pub fetched_at: u64,
    pub graph: ResolvedGraph,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtractedPackageMarker {
    #[serde(default)]
    pub schema_version: u32,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub integrity: Option<String>,
    pub tarball_sha256: String,
    #[serde(default)]
    pub file_count: u64,
    #[serde(default)]
    pub unpacked_size: u64,
    #[serde(default)]
    pub file_tree_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TarballContentSignature {
    pub file_count: u64,
    pub unpacked_size: u64,
    pub file_tree_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SharedCacheProjectRef {
    pub schema_version: u32,
    pub project_root: String,
    pub updated_at: u64,
    pub package_roots: Vec<String>,
}
