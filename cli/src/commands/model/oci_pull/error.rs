// OCI pull failures — lỗi tải OCI.
// Own OCI locator and manifest request diagnostics — sở hữu lỗi locator OCI và yêu cầu manifest.

use std::fmt;

#[derive(Debug)]
pub(in crate::commands::model) enum OciPullError {
    InvalidSource(String),
    ManifestPullFailed,
}

impl fmt::Display for OciPullError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSource(source) => {
                write!(
                    f,
                    "invalid oci source '{source}' — use `oci://registry/repo:tag`"
                )
            }
            Self::ManifestPullFailed => {
                f.write_str("pull manifest failed (is the registry running? `mgc registry serve`)")
            }
        }
    }
}

impl std::error::Error for OciPullError {}
