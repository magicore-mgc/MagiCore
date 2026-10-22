//! Public data model for a resolvable Gradle version-catalog library.
//! Mô hình dữ liệu công khai cho library resolve được trong version catalog Gradle.

/// A version-catalog library entry.
/// Một entry library trong version catalog.
#[derive(Debug, Clone)]
pub struct CatalogEntry {
    /// Alias in `[libraries]` (e.g. `commons-lang3`).
    /// Alias trong `[libraries]` (ví dụ `commons-lang3`).
    pub alias: String,
    /// Maven group (e.g. `org.apache.commons`).
    /// Maven group (ví dụ `org.apache.commons`).
    pub group: String,
    /// Maven artifact (e.g. `commons-lang3`).
    /// Maven artifact (ví dụ `commons-lang3`).
    pub artifact: String,
    /// Pinned version string, resolved through `version.ref` when present.
    /// Chuỗi version đã pin, resolve qua `version.ref` nếu có.
    pub version: String,
    /// The `[versions]` ref name when versioned by reference; `None` for inline literals.
    /// Tên ref tại `[versions]` nếu version dùng tham chiếu; `None` với version inline.
    pub version_ref: Option<String>,
}
