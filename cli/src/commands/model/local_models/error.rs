// Local model failures — lỗi model lưu cục bộ.
// Report a missing local model and its manifest location — báo model không có cùng vị trí manifest.

use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub(in crate::commands::model) enum LocalModelError {
    NotFound { name: String, manifest: PathBuf },
}

impl fmt::Display for LocalModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound { name, manifest } => write!(
                f,
                "model '{name}' not found locally (manifest: {})",
                manifest.display()
            ),
        }
    }
}

impl std::error::Error for LocalModelError {}
