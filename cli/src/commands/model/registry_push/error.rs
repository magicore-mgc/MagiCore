// Registry push failures — lỗi đẩy model lên registry.
// Classify input-path and registry push failures — phân loại lỗi đường dẫn và push registry.

use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub(in crate::commands::model) enum RegistryPushError {
    PathNotFound(PathBuf),
    DirectoryHasNoFiles(PathBuf),
    PushFailed,
}

impl fmt::Display for RegistryPushError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PathNotFound(path) => write!(f, "path does not exist: {}", path.display()),
            Self::DirectoryHasNoFiles(path) => {
                write!(f, "directory has no files: {}", path.display())
            }
            Self::PushFailed => {
                f.write_str("push model failed — is the server running? (`mgc registry serve`)")
            }
        }
    }
}

impl std::error::Error for RegistryPushError {}
