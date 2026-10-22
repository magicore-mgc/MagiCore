//! I/O and JSON error contract for workspace manifest reading.
//! Hợp đồng lỗi I/O và JSON khi đọc manifest workspace.

/// Errors that the current reader propagates to its public anyhow boundary.
/// Các lỗi hiện được bộ đọc truyền tới ranh giới anyhow public.
#[derive(Debug)]
pub(in crate::graph::manifest_reader) enum ManifestReadError {
    Io(std::io::Error),
    PackageJson(serde_json::Error),
}

impl ManifestReadError {
    /// Preserve the public anyhow chain by returning its original source error.
    /// Giữ nguyên chuỗi anyhow public bằng cách trả lỗi nguồn ban đầu.
    pub(in crate::graph::manifest_reader) fn into_anyhow(self) -> anyhow::Error {
        match self {
            Self::Io(source) => anyhow::Error::new(source),
            Self::PackageJson(source) => anyhow::Error::new(source),
        }
    }
}

impl std::fmt::Display for ManifestReadError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(source) => std::fmt::Display::fmt(source, formatter),
            Self::PackageJson(source) => std::fmt::Display::fmt(source, formatter),
        }
    }
}

impl std::error::Error for ManifestReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(source) => Some(source),
            Self::PackageJson(source) => Some(source),
        }
    }
}

impl From<std::io::Error> for ManifestReadError {
    fn from(source: std::io::Error) -> Self {
        Self::Io(source)
    }
}

impl From<serde_json::Error> for ManifestReadError {
    fn from(source: serde_json::Error) -> Self {
        Self::PackageJson(source)
    }
}
