//! Installed package identity and materialized path returned by adapters. — Identity package đã cài và đường dẫn materialize do adapter trả về.

use std::path::PathBuf;

use crate::package::PackageId;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InstalledPackage {
    pub id: PackageId,
    pub path: PathBuf,
    pub integrity: Option<String>,
    pub is_direct: bool,
    pub is_dev: bool,
}
