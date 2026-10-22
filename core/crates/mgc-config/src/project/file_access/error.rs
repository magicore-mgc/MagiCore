//! Typed errors for shared project-file operations.
//! Lỗi typed cho thao tác file project dùng chung.
use std::error::Error as StdError;
use std::fmt;
use std::path::PathBuf;

pub(super) type Result<T> = std::result::Result<T, FileAccessError>;

#[derive(Debug)]
pub(super) enum FileAccessError {
    Source(anyhow::Error),
    Symlink {
        label: String,
        path: PathBuf,
    },
    NotRegular {
        label: String,
        path: PathBuf,
    },
    OpenNoFollow {
        label: String,
        path: PathBuf,
        source: std::io::Error,
    },
    OpenedNotRegular {
        label: String,
        path: PathBuf,
    },
    TooLarge {
        label: String,
        path: PathBuf,
        limit: u64,
    },
    #[cfg(windows)]
    ReparsePoint {
        label: String,
        path: PathBuf,
    },
    MissingFileName,
}

impl FileAccessError {
    /// Preserve the existing anyhow boundary: original sources pass through;
    /// local validation failures retain their established message text.
    /// Giữ biên anyhow hiện tại: source gốc đi xuyên qua, lỗi validation giữ nguyên message.
    pub(super) fn into_anyhow(self) -> anyhow::Error {
        match self {
            Self::Source(error) => error,
            domain_error => anyhow::Error::msg(domain_error.to_string()),
        }
    }
}

impl fmt::Display for FileAccessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Source(error) => write!(formatter, "{error}"),
            Self::Symlink { label, path } => {
                write!(
                    formatter,
                    "{label} '{}' must not be a symlink",
                    path.display()
                )
            }
            Self::NotRegular { label, path } => {
                write!(
                    formatter,
                    "{label} '{}' must be a regular file",
                    path.display()
                )
            }
            Self::OpenNoFollow {
                label,
                path,
                source,
            } => write!(
                formatter,
                "open {label} '{}' without following links: {source}",
                path.display()
            ),
            Self::OpenedNotRegular { label, path } => {
                write!(
                    formatter,
                    "{label} '{}' is not a regular file",
                    path.display()
                )
            }
            Self::TooLarge { label, path, limit } => write!(
                formatter,
                "{label} '{}' exceeds the {limit} byte safety limit",
                path.display()
            ),
            #[cfg(windows)]
            Self::ReparsePoint { label, path } => write!(
                formatter,
                "{label} '{}' must not be a reparse point",
                path.display()
            ),
            Self::MissingFileName => formatter.write_str("project config path has no file name"),
        }
    }
}

impl StdError for FileAccessError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Source(error) => Some(error.as_ref()),
            Self::OpenNoFollow { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl From<anyhow::Error> for FileAccessError {
    fn from(error: anyhow::Error) -> Self {
        Self::Source(error)
    }
}

impl From<std::io::Error> for FileAccessError {
    fn from(error: std::io::Error) -> Self {
        Self::Source(error.into())
    }
}
