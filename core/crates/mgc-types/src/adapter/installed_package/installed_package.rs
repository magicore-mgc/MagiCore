//! Installed package identity, materialized path and manifest section. — Identity package đã cài, đường dẫn materialize và section manifest.

use std::path::PathBuf;

use crate::package::PackageId;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InstalledPackage {
    pub id: PackageId,
    pub path: PathBuf,
    pub integrity: Option<String>,
    pub is_direct: bool,
    pub is_dev: bool,
    /// Manifest section the package is declared in (`dependencies`,
    /// `devDependencies`, `optionalDependencies`, `peerDependencies`).
    /// `None` when the adapter cannot attribute a section (transitive or
    /// engine-owned inventory), so `list` prints one only when known.
    /// (Section manifest chứa package khai báo. `None` khi adapter không xác
    /// định được section (transitive hoặc inventory do engine sở hữu), nên
    /// `list` chỉ in khi biết.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
}
