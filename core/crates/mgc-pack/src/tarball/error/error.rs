//! Deterministic tarball policy and size errors.
//! Lỗi policy tarball và kích thước xác định.

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(in crate::tarball) enum PackError {
    ArchiveOutputMustNameFile,
    PackageInputNotRegular(String),
    #[cfg(windows)]
    PackageInputReparsePoint(String),
    PackageInputShrank(String),
    PackageInputGrew(String),
    PackageUnpackedSizeOverflow,
    ArchiveSizeOverflow,
}

impl Display for PackError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArchiveOutputMustNameFile => {
                formatter.write_str("Archive output must name a file")
            }
            Self::PackageInputNotRegular(path) => {
                write!(formatter, "Package input must be a regular file: {path}")
            }
            #[cfg(windows)]
            Self::PackageInputReparsePoint(path) => {
                write!(
                    formatter,
                    "Package input must not be a reparse point: {path}"
                )
            }
            Self::PackageInputShrank(path) => {
                write!(formatter, "Package input shrank while packing: {path}")
            }
            Self::PackageInputGrew(path) => {
                write!(formatter, "Package input grew while packing: {path}")
            }
            Self::PackageUnpackedSizeOverflow => {
                formatter.write_str("Package unpacked size overflow")
            }
            Self::ArchiveSizeOverflow => formatter.write_str("Archive size overflow"),
        }
    }
}

impl std::error::Error for PackError {}
