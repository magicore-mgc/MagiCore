//! Typed user-facing failures for Web dependency explanation.
//! Lỗi hiển thị có kiểu cho lệnh giải thích phụ thuộc Web.

use std::fmt;

#[derive(Debug)]
pub(super) enum WhyError {
    UnsupportedCore,
    LockMissing,
    PackageNotPinned { package: String },
}

impl WhyError {
    pub(super) fn unsupported_core() -> Self {
        Self::UnsupportedCore
    }

    pub(super) fn lock_missing() -> Self {
        Self::LockMissing
    }

    pub(super) fn package_not_pinned(package: &str) -> Self {
        Self::PackageNotPinned {
            package: package.to_owned(),
        }
    }
}

impl fmt::Display for WhyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedCore => formatter.write_str("why only supported for web core"),
            Self::LockMissing => {
                formatter.write_str("mgc.lock not found — run 'mgc install' first")
            }
            Self::PackageNotPinned { package } => write!(
                formatter,
                "`mgc why {package}`: '{package}' is not pinned in mgc.lock — nothing in this project can depend on it."
            ),
        }
    }
}

impl std::error::Error for WhyError {}
