// OCI pull failures — lỗi tải OCI.
// Own OCI locator and manifest request diagnostics — sở hữu lỗi locator OCI và yêu cầu manifest.

use std::fmt;

#[derive(Debug)]
pub(in crate::commands::model) enum OciPullError {
    InvalidSource(String),
    ManifestPullFailed,
    /// The image reference carries no tag and no digest (PAR-12).
    /// Tham chiếu image không có tag và không có digest (PAR-12).
    UnpinnedReference(String),
}

impl fmt::Display for OciPullError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnpinnedReference(source) => write!(
                f,
                "refusing to pull '{source}': pass an explicit tag (repo:tag) or, better, \
                 a digest (repo@sha256:...), so the model can be reproduced later"
            ),
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
