//! Resolved registry package identity, dependency edges and integrity metadata.
//! Danh tính package đã resolve, cạnh dependency và metadata toàn vẹn.

/// Resolution payload a native engine returns for one package.
/// Payload resolve mà engine native trả về cho một package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedEntry {
    /// Package name at the registry — Tên package trên registry.
    pub name: String,
    /// Exact resolved version — Version chính xác đã resolve.
    pub version: String,
    /// Resolved dependency edges as `(name, range)`. Crates.io includes dormant
    /// optional and target-qualified edges in its complete Cargo lock closure;
    /// selected features remain represented separately by markers.
    /// Cạnh dependency đã resolve dạng `(name, range)`. Crates.io giữ cả cạnh
    /// optional dormant và theo target trong lock closure Cargo; feature được
    /// chọn vẫn phân biệt bằng marker.
    pub deps: Vec<(String, String)>,
    /// Artifact download URL — URL tải artifact.
    pub artifact_url: String,
    /// Declared sha256 (lowercase hex; empty when the registry gave none).
    /// Sha256 khai báo (hex thường; rỗng khi registry không cung cấp).
    pub sha256: String,
    /// Extra markers (cfg tags, wheel tags, optional-dep names, PEP 508 markers).
    /// Marker thêm (tag cfg, tag wheel, tên optional-dep, marker PEP 508).
    pub extra_markers: Vec<String>,
}
