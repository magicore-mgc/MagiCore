//! Typed failures for archive paths, limits, I/O and CAS publication.
//! Lỗi typed cho path, giới hạn, I/O và publish CAS.

use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub(in crate::extract) enum ExtractionError {
    Io(io::Error),
    EntryCountOverflow,
    EntryLimit,
    EntryTooLarge,
    TotalSizeOverflow,
    TotalSizeLimit,
    UnsafeArchivePath(PathBuf),
    EmptyArchivePath,
    PathEscapes(PathBuf),
    LinksNotAllowed(PathBuf),
    UnsupportedEntry(PathBuf),
    RootSymlink(PathBuf),
    RootNotDirectory(PathBuf),
    ParentSymlink(PathBuf),
    ParentNotDirectory(PathBuf),
    TargetSymlink(PathBuf),
    TargetNotFile(PathBuf),
    DestinationHasNoParent,
    TruncatedEntry(PathBuf),
    InstallHookNotAllowed { path: PathBuf, reason: &'static str },
    CasImport(anyhow::Error),
    CasExport(anyhow::Error),
    Claim(anyhow::Error),
}

pub(in crate::extract) type ExtractionResult<T> = Result<T, ExtractionError>;

impl fmt::Display for ExtractionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => fmt::Display::fmt(error, formatter),
            Self::EntryCountOverflow => formatter.write_str("tar archive entry count overflow"),
            Self::EntryLimit => write!(
                formatter,
                "tar archive exceeds the {} entry limit",
                super::super::archive_limits::MAX_ARCHIVE_ENTRIES
            ),
            Self::EntryTooLarge => write!(
                formatter,
                "tar entry exceeds the {} byte limit",
                super::super::archive_limits::MAX_ENTRY_BYTES
            ),
            Self::TotalSizeOverflow => {
                formatter.write_str("tar archive uncompressed size overflow")
            }
            Self::TotalSizeLimit => write!(
                formatter,
                "tar archive exceeds the {} byte uncompressed size limit",
                super::super::archive_limits::MAX_TOTAL_FILE_BYTES
            ),
            Self::UnsafeArchivePath(path) => {
                write!(formatter, "unsafe tar entry path: {}", path.display())
            }
            Self::EmptyArchivePath => formatter.write_str("empty tar entry path"),
            Self::PathEscapes(path) => write!(
                formatter,
                "tar entry escapes destination: {}",
                path.display()
            ),
            Self::LinksNotAllowed(path) => {
                write!(formatter, "tar links are not allowed: {}", path.display())
            }
            Self::UnsupportedEntry(path) => write!(
                formatter,
                "unsupported tar entry type for {}",
                path.display()
            ),
            Self::RootSymlink(path) => write!(
                formatter,
                "refusing linked tar extraction root: {}",
                path.display()
            ),
            Self::RootNotDirectory(path) => write!(
                formatter,
                "refusing non-directory tar extraction root: {}",
                path.display()
            ),
            Self::ParentSymlink(path) => write!(
                formatter,
                "refusing linked tar extraction parent: {}",
                path.display()
            ),
            Self::ParentNotDirectory(path) => write!(
                formatter,
                "refusing non-directory tar extraction parent: {}",
                path.display()
            ),
            Self::TargetSymlink(path) => write!(
                formatter,
                "refusing tar extraction target symlink: {}",
                path.display()
            ),
            Self::TargetNotFile(path) => write!(
                formatter,
                "refusing non-file tar extraction target: {}",
                path.display()
            ),
            Self::DestinationHasNoParent => {
                formatter.write_str("extraction destination has no parent directory")
            }
            Self::TruncatedEntry(path) => {
                write!(formatter, "tar entry '{}' is truncated", path.display())
            }
            Self::InstallHookNotAllowed { path, reason } => write!(
                formatter,
                "refusing archive entry '{}': {reason} (set {} for an explicit package to allow it)",
                path.display(),
                super::super::install_hook_guard::ALLOW_INSTALL_HOOKS_ENV
            ),
            Self::CasImport(error) => write!(formatter, "failed to import to CAS: {error}"),
            Self::CasExport(error) => write!(formatter, "failed to export from CAS: {error}"),
            Self::Claim(error) => fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::error::Error for ExtractionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::CasImport(error) | Self::CasExport(error) | Self::Claim(error) => {
                Some(error.as_ref())
            }
            _ => None,
        }
    }
}

impl From<io::Error> for ExtractionError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
