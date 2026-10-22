// Package build-cache record — bản ghi cache build package

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Metadata for each package's workspace computation cache.
/// Metadata lưu trữ cache computation của từng package trong workspace.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PackageBuildCache {
    /// Combined hash of source code and local configuration.
    /// Hash tổng hợp của source code và cấu hình local.
    pub source_hash: String,
    /// Hashes of dependencies from other workspace packages.
    /// Danh sách hash dependency từ các package khác trong workspace.
    pub dependency_hashes: BTreeMap<String, String>,
    /// Combined computation hash đại diện cho toàn bộ build state
    pub composite_hash: String,
    /// Build completion timestamp in ISO 8601 format.
    /// Thời gian build hoàn tất theo định dạng ISO 8601.
    pub last_built_at: String,
}
