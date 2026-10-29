//! Packagist p2 metadata shapes (Composer v2 repository document).
//! Cấu trúc metadata p2 của Packagist (tài liệu repository Composer v2).

use serde::Deserialize;
use std::collections::BTreeMap;

/// Composer v2 repository document: `{"minified":..,"packages":{..}}`.
/// Field thừa của Composer (autoload, authors, funding…) vẫn qua được vì
/// serde chỉ đọc những khoá được khai báo.
#[derive(Debug, Deserialize)]
pub(in crate::protocols::composer) struct PackagistDocument {
    /// Package lists keyed by lowercase `vendor/package`.
    /// Danh sách package khoá theo `vendor/package` chữ thường.
    #[serde(default)]
    pub(in crate::protocols::composer) packages: BTreeMap<String, Vec<PackagistVersion>>,
}

/// One released version entry in a Packagist package list.
/// Một entry version đã phát hành trong danh sách package Packagist.
#[derive(Debug, Clone, Deserialize)]
pub(in crate::protocols::composer) struct PackagistVersion {
    #[serde(default)]
    pub(in crate::protocols::composer) version: String,
    #[serde(default)]
    pub(in crate::protocols::composer) dist: Option<PackagistDist>,
    /// Runtime dependencies (`require`).
    /// Phụ thuộc runtime (`require`).
    #[serde(default)]
    pub(in crate::protocols::composer) require: BTreeMap<String, String>,
    /// Development-only dependencies (`require-dev`).
    /// Phụ thuộc chỉ dùng khi phát triển (`require-dev`).
    #[serde(default, rename = "require-dev")]
    pub(in crate::protocols::composer) require_dev: BTreeMap<String, String>,
}

/// Distribution archive for one version.
/// Archive phân phối của một version.
#[derive(Debug, Clone, Deserialize)]
pub(in crate::protocols::composer) struct PackagistDist {
    #[serde(default)]
    pub(in crate::protocols::composer) url: String,
    #[serde(default, rename = "type")]
    pub(in crate::protocols::composer) archive_type: String,
    /// sha1 hex of the archive; Packagist commonly leaves it empty.
    /// sha1 hex của archive; Packagist thường để rỗng.
    #[serde(default)]
    pub(in crate::protocols::composer) shasum: Option<String>,
    /// VCS commit reference the archive was built from.
    /// Commit reference VCS mà archive được dựng từ đó.
    #[serde(default)]
    pub(in crate::protocols::composer) reference: Option<String>,
}
