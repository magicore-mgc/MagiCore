//! Sanitized package manifest result — carries the final manifest and identity.
//! Kết quả manifest đã làm sạch — mang manifest cùng định danh cuối cùng.

use serde_json::Value;

/// Exportable manifest and publish identity after validation and overrides.
/// Manifest có thể publish và định danh sau khi kiểm tra, áp dụng override.
pub struct Sanitized {
    /// Manifest JSON with publish-only fields removed.
    /// JSON manifest đã bỏ các trường chỉ dành cho cấu hình publish.
    pub manifest: Value,
    /// Package name selected for the publish payload.
    /// Tên package dùng cho payload publish.
    pub name: String,
    /// Package version selected for the publish payload.
    /// Version package dùng cho payload publish.
    pub version: String,
}
