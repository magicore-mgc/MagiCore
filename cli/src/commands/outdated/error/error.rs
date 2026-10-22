//! Typed failures for native outdated graph comparisons.
//! Lỗi typed cho so sánh graph outdated native.

use std::fmt;

#[derive(Debug)]
pub(in crate::commands::outdated) enum OutdatedError {
    CurrentDirectCardinality {
        package: String,
        found: usize,
    },
    LatestDirectCardinality {
        package: String,
        found: usize,
    },
    Downgrade {
        package: String,
        current: String,
        latest: String,
    },
}

impl fmt::Display for OutdatedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CurrentDirectCardinality { package, found } => write!(
                f,
                "native outdated needs exactly one current locked version for direct dependency '{package}' (found {found})"
            ),
            Self::LatestDirectCardinality { package, found } => write!(
                f,
                "native outdated resolver did not return exactly one latest version for direct dependency '{package}' (found {found})"
            ),
            Self::Downgrade {
                package,
                current,
                latest,
            } => write!(
                f,
                "native outdated resolver returned {latest} below locked version {current} for '{package}'; refusing to report a downgrade"
            ),
        }
    }
}

impl std::error::Error for OutdatedError {}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
