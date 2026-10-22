// Manifest path failures — lỗi đường dẫn manifest.
// Reject unsafe names and path components — từ chối tên và thành phần đường dẫn không an toàn.

use std::fmt;

#[derive(Debug)]
pub(in crate::commands::model) enum ManifestPathError {
    InvalidModelName(String),
    UnsafeManifestPath,
}

impl fmt::Display for ManifestPathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidModelName(name) => {
                write!(f, "model name must be a safe relative path: '{name}'")
            }
            Self::UnsafeManifestPath => {
                f.write_str("model manifest path contains a symlink or non-directory component")
            }
        }
    }
}

impl std::error::Error for ManifestPathError {}
