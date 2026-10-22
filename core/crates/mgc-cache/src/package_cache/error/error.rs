//! Typed cache failures with stable user-facing messages.
//! Lỗi cache có kiểu và thông báo ổn định cho người dùng.

use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub(in crate::package_cache) enum LockOperation {
    Read,
    Write,
    Prune,
}

#[derive(Debug)]
pub(in crate::package_cache) enum CacheError {
    LockPoisoned {
        package_id: String,
        operation: LockOperation,
    },
    PackageNotInCache(String),
    IntegrityMismatch {
        package_id: String,
        expected: String,
        actual: String,
        during_store: bool,
    },
    UnsafeCachePath(PathBuf),
    TemporaryFileExhausted(String),
}

impl fmt::Display for CacheError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LockPoisoned {
                package_id,
                operation,
            } => {
                let action = match operation {
                    LockOperation::Read => "reading",
                    LockOperation::Write => "writing",
                    LockOperation::Prune => "pruning",
                };
                write!(
                    formatter,
                    "cache lock poisoned for '{package_id}' — refusing rather than {action} uncertain state"
                )
            }
            Self::PackageNotInCache(package_id) => {
                write!(formatter, "Package not in cache: {package_id}")
            }
            Self::IntegrityMismatch {
                package_id,
                expected,
                actual,
                during_store,
            } => {
                if *during_store {
                    write!(
                        formatter,
                        "Integrity mismatch during store for {package_id}: expected {expected}, got {actual}"
                    )
                } else {
                    write!(
                        formatter,
                        "Integrity mismatch for {package_id}: expected {expected}, got {actual}"
                    )
                }
            }
            Self::UnsafeCachePath(path) => write!(
                formatter,
                "Refusing to access an unsafe cache path: {}",
                path.display()
            ),
            Self::TemporaryFileExhausted(package_id) => write!(
                formatter,
                "Could not create a unique temporary cache file for {package_id}"
            ),
        }
    }
}

impl std::error::Error for CacheError {}
