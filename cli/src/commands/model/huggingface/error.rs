// Hugging Face failures — lỗi tải Hugging Face.
// Own validation, integrity and bounded-download diagnostics — sở hữu lỗi kiểm tra, toàn vẹn và quota tải.

use std::fmt;

#[derive(Debug)]
pub(in crate::commands::model) enum HuggingFaceError {
    RequestFailed,
    DownloadFailed { status: String, url: String },
    ArtifactExceedsLimit(u64),
    RevisionRequired,
    InvalidRevision,
    Sha256Required,
    InvalidSha256,
    InvalidSource,
    SizeOverflow,
    DownloadLimitExceeded(u64),
    ContentLengthMismatch { expected: u64, actual: u64 },
    Sha256Mismatch { expected: String, actual: String },
    ArtifactChangedDuringImport,
    UnableToConstructUrl,
}

impl fmt::Display for HuggingFaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestFailed => f.write_str("HF request failed"),
            Self::DownloadFailed { status, url } => write!(
                f,
                "HF download failed: {status} ({url}) — artifact was not written to the store"
            ),
            Self::ArtifactExceedsLimit(limit) => write!(
                f,
                "HF artifact exceeds --max-bytes ({limit}); no artifact was written"
            ),
            Self::RevisionRequired => {
                f.write_str("hf:// pulls require --revision with an immutable 40-hex commit SHA")
            }
            Self::InvalidRevision => {
                f.write_str("HF revision must be an immutable 40-hex commit SHA")
            }
            Self::Sha256Required => {
                f.write_str("hf:// pulls require --sha256 with the trusted 64-hex artifact digest")
            }
            Self::InvalidSha256 => {
                f.write_str("HF SHA-256 must contain exactly 64 hexadecimal characters")
            }
            Self::InvalidSource => {
                f.write_str("invalid hf source; expected hf://<org>/<model>/<file>")
            }
            Self::SizeOverflow => f.write_str("HF artifact size overflow"),
            Self::DownloadLimitExceeded(limit) => write!(
                f,
                "HF artifact exceeded --max-bytes ({limit}); download was not imported into CAS"
            ),
            Self::ContentLengthMismatch { expected, actual } => write!(
                f,
                "HF artifact Content-Length mismatch: expected {expected}, received {actual}"
            ),
            Self::Sha256Mismatch { expected, actual } => {
                write!(
                    f,
                    "HF artifact SHA-256 mismatch: expected {expected}, got {actual}"
                )
            }
            Self::ArtifactChangedDuringImport => {
                f.write_str("HF artifact changed while being imported into CAS")
            }
            Self::UnableToConstructUrl => f.write_str("unable to construct HF artifact URL"),
        }
    }
}

impl std::error::Error for HuggingFaceError {}
