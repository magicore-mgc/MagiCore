// Offline error taxonomy — typed failures local to the offline owner.
// Phân loại lỗi offline — lỗi có kiểu riêng trong owner offline.

use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub(super) enum OfflineError {
    CacheMiss { url: String },
    MethodNotAllowed { method: &'static str },
    ReadCache(std::io::Error),
    ParseCache(serde_json::Error),
    SerializeCache(serde_json::Error),
    WriteCache(std::io::Error),
}

impl Display for OfflineError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::CacheMiss { url } => {
                write!(
                    formatter,
                    "Offline mode: no cached data for {url} - E_NET_OFFLINE"
                )
            }
            Self::MethodNotAllowed { method } => {
                write!(formatter, "Offline mode: {method} not allowed")
            }
            Self::ReadCache(error) => write!(formatter, "read cache file: {error}"),
            Self::ParseCache(error) => write!(formatter, "parse cache JSON: {error}"),
            Self::SerializeCache(error) => Display::fmt(error, formatter),
            Self::WriteCache(error) => Display::fmt(error, formatter),
        }
    }
}

impl Error for OfflineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ReadCache(error) => Some(error),
            Self::WriteCache(error) => Some(error),
            Self::ParseCache(error) => Some(error),
            Self::SerializeCache(error) => Some(error),
            Self::CacheMiss { .. } | Self::MethodNotAllowed { .. } => None,
        }
    }
}
