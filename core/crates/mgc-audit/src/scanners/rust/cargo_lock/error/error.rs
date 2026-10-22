//! Typed errors owned by the Cargo.lock parser.
//! Lỗi typed do parser Cargo.lock sở hữu.

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub(in crate::scanners::rust) enum CargoLockError {
    InvalidToml(toml::de::Error),
    PackageFieldNotArray,
    PackageNameInvalid,
    PackageVersionInvalid { package_name: String },
}

impl fmt::Display for CargoLockError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidToml(error) => write!(formatter, "invalid Cargo.lock TOML: {error}"),
            Self::PackageFieldNotArray => {
                formatter.write_str("Cargo.lock package field is not an array")
            }
            Self::PackageNameInvalid => formatter.write_str("Cargo.lock package has no valid name"),
            Self::PackageVersionInvalid { package_name } => write!(
                formatter,
                "Cargo.lock package '{package_name}' has no valid version"
            ),
        }
    }
}

impl Error for CargoLockError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidToml(error) => Some(error),
            Self::PackageFieldNotArray
            | Self::PackageNameInvalid
            | Self::PackageVersionInvalid { .. } => None,
        }
    }
}
