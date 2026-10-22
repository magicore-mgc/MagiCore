//! Public file entry and private serialized index records.
//! FileEntry công khai và các record chỉ mục serialize nội bộ.

use serde::{Deserialize, Serialize};

use mgc_types::PackageId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileEntry {
    pub path: String,
    pub blob_hash: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(in crate::index) struct PackageIndex {
    pub(in crate::index) id: String,
    pub(in crate::index) version: String,
    pub(in crate::index) files: Vec<FileEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(in crate::index) struct IndexFile {
    pub(in crate::index) version: u32,
    pub(in crate::index) checksum: String,
    pub(in crate::index) packages: Vec<PackageIndex>,
}

pub(in crate::index) fn package_key(id: &PackageId) -> String {
    format!("{}#{}", id.name_str(), id.version())
}
