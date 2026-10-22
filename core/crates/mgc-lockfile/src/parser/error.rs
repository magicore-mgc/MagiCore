//! Typed schema-dispatch failures mapped to the stable lockfile error API.
//! Lỗi dispatch schema có kiểu, ánh xạ về API lỗi lockfile ổn định.

/// Failure while decoding the lockfile schema document.
/// Lỗi khi giải mã tài liệu schema lockfile.
#[derive(Debug, thiserror::Error)]
pub(super) enum DocumentParseError {
    #[error("TOML parse failed: {0}")]
    Toml(#[source] toml::de::Error),
    #[error("missing lockfile version")]
    MissingVersion,
    #[error("unsupported lockfile version: {0}")]
    UnsupportedVersion(String),
    #[error(transparent)]
    V4Revision(#[from] crate::LockfileError),
}

impl From<DocumentParseError> for crate::LockfileError {
    fn from(error: DocumentParseError) -> Self {
        match error {
            DocumentParseError::V4Revision(source) => source,
            other => Self::ParseError(other.to_string()),
        }
    }
}
