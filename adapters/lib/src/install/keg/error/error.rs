//! Typed errors for the lib keg/prefix transfer layer.
//! Lỗi typed cho lớp keg/prefix của core lib.

/// Failures raised by keg, prefix and switch operations.
/// Lỗi phát sinh từ thao tác keg, prefix và switch.
///
/// Every variant keeps the path that failed so CLI output stays actionable
/// without string matching. Messages are English (RULE §7).
/// Mọi variant giữ đường dẫn lỗi để CLI báo được cụ thể, không match chuỗi.
#[derive(Debug, thiserror::Error)]
pub enum KegError {
    /// Resolving MagiCore data paths failed.
    /// Resolve đường dẫn dữ liệu MagiCore thất bại.
    #[error("cannot resolve MagiCore layout paths: {0}")]
    Layout(String),

    /// A keg name or relative path escaped the keg root.
    /// Tên keg hoặc đường dẫn tương đối vượt ra ngoài keg root.
    #[error("unsafe keg path component in '{path}': {reason}")]
    UnsafePath { path: String, reason: &'static str },

    /// The requested version is not a plain semantic version.
    /// Version yêu cầu không phải semver thuần.
    #[error("invalid keg version '{0}': expected a plain semantic version")]
    InvalidVersion(String),

    /// A binary manifest is missing, untrusted or does not verify (E-3).
    /// Manifest binary thiếu, không tin cậy hoặc không verify (E-3).
    #[error("binary manifest rejected: {detail}")]
    ManifestUntrusted { detail: String },

    /// Downloaded bytes do not match the digest in the signed manifest.
    /// Bytes tải về không khớp digest trong manifest đã ký.
    #[error("downloaded artifact '{name}' does not match the signed digest")]
    ArtifactDigestMismatch { name: String },

    /// The requested keg does not exist.
    /// Keg yêu cầu không tồn tại.
    #[error("library keg '{name}@{version}' not found at '{path}'")]
    KegNotFound {
        name: String,
        version: String,
        path: String,
    },

    /// A keg with this version already exists; reinstalling is refused.
    /// Keg version này đã tồn tại; cài lại bị từ chối.
    #[error("library keg '{name}@{version}' already exists at '{path}'")]
    KegExists {
        name: String,
        version: String,
        path: String,
    },

    /// The keg directory was modified after it was sealed.
    /// Thư mục keg bị sửa sau khi đã seal.
    #[error("keg '{path}' was modified after sealing: {detail}")]
    Tampered { path: String, detail: String },

    /// The active-version pointer is unreadable or outside the keg list.
    /// Con trỏ version active không đọc được hoặc không thuộc danh sách keg.
    #[error("invalid active version pointer for '{name}' at '{path}': {reason}")]
    InvalidActive {
        name: String,
        path: String,
        reason: String,
    },

    /// No active version/prefix exists for the library.
    /// Chưa có version active/prefix cho library.
    #[error("no active version for '{0}'; run an install or switch first")]
    NoActiveVersion(String),

    /// A prefix entry has no counterpart in the active keg.
    /// Entry prefix không có tương ứng trong keg active.
    #[error("prefix entry '{entry}' has no file in keg '{keg}'")]
    PrefixEntryMissing { entry: String, keg: String },

    /// A filesystem operation failed; wrapped with the operation context.
    /// Thao tác filesystem thất bại; kèm ngữ cảnh thao tác.
    #[error("{operation} '{}': {source}", path.display())]
    Io {
        operation: &'static str,
        path: std::path::PathBuf,
        source: std::io::Error,
    },
}

impl KegError {
    /// Wrap an IO failure with the operation and path that failed.
    /// Bọc lỗi IO kèm thao tác và đường dẫn lỗi.
    pub(crate) fn io(
        operation: &'static str,
        path: impl Into<std::path::PathBuf>,
        source: std::io::Error,
    ) -> Self {
        Self::Io {
            operation,
            path: path.into(),
            source,
        }
    }
}

impl From<std::io::Error> for KegError {
    /// Bare IO errors (test fixtures, setup helpers) keep their OS message.
    /// Lỗi IO trần (fixture test, helper setup) giữ thông điệp OS.
    fn from(source: std::io::Error) -> Self {
        Self::Io {
            operation: "io",
            path: std::path::PathBuf::from("<unspecified>"),
            source,
        }
    }
}

/// Result alias for the keg/prefix transfer layer.
/// Alias Result cho lớp keg/prefix.
pub type KegResult<T> = Result<T, KegError>;
