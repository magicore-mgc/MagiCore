//! Typed package-entry resolution failures.
//! Lỗi typed khi resolve package entry.

#[derive(Debug, thiserror::Error)]
pub(in crate::bundler::dependency_cache) enum PackageEntryError {
    #[error("invalid installed package name")]
    InvalidPackageName,
    #[error("package entry path is invalid or outside its package root")]
    ForbiddenPath,
    #[error("failed to inspect installed package files")]
    Io(#[source] std::io::Error),
}
