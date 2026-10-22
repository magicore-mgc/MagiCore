//! Locked model file fields.
//! Các trường file model đã lock.

/// One locked model file.
/// Một file model đã lock.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelFile {
    /// Repo-relative path.
    /// Đường dẫn tương đối trong repo.
    pub path: String,
    /// Byte size when the registry reports it.
    /// Kích thước byte khi registry cung cấp.
    pub size: Option<u64>,
    /// LFS SHA-256 when present.
    /// SHA-256 LFS nếu có.
    pub sha256: Option<String>,
    /// Git blob id in hexadecimal.
    /// Git blob id dạng hex.
    pub git_blob_sha1: String,
    /// Direct download URL for the pinned revision.
    /// URL tải trực tiếp theo revision đã ghim.
    pub url: String,
}
