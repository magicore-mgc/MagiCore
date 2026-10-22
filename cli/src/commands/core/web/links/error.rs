//! Typed failures at the Web package-link boundary.
//! Lỗi có kiểu tại boundary link package Web.

use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub(super) enum WebLinkError {
    UnsupportedCore { command: &'static str },
    MissingPackage { command: &'static str },
    LinkExists { name: String },
    UnlinkNotLinked { name: String },
    NotPackageLink { name: String },
    InvalidLinkName { package: String },
    UnsafeLinkParent { package: String },
    LocalPathNotFound { path: PathBuf },
    PackageNotFound { package: String },
    Filesystem { source: io::Error },
}

impl WebLinkError {
    pub(super) fn unsupported_core(command: &'static str) -> Self {
        Self::UnsupportedCore { command }
    }

    pub(super) fn missing_package(command: &'static str) -> Self {
        Self::MissingPackage { command }
    }

    pub(super) fn link_exists(name: &str) -> Self {
        Self::LinkExists {
            name: name.to_owned(),
        }
    }

    pub(super) fn unlink_not_linked(name: &str) -> Self {
        Self::UnlinkNotLinked {
            name: name.to_owned(),
        }
    }

    pub(super) fn not_package_link(name: &str) -> Self {
        Self::NotPackageLink {
            name: name.to_owned(),
        }
    }

    pub(super) fn invalid_link_name(package: &str) -> Self {
        Self::InvalidLinkName {
            package: package.to_owned(),
        }
    }

    pub(super) fn unsafe_link_parent(package: &str) -> Self {
        Self::UnsafeLinkParent {
            package: package.to_owned(),
        }
    }

    pub(super) fn local_path_not_found(path: &std::path::Path) -> Self {
        Self::LocalPathNotFound {
            path: path.to_path_buf(),
        }
    }

    pub(super) fn package_not_found(package: &str) -> Self {
        Self::PackageNotFound {
            package: package.to_owned(),
        }
    }

    pub(super) fn filesystem(source: io::Error) -> Self {
        Self::Filesystem { source }
    }
}

impl fmt::Display for WebLinkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedCore { command } => {
                write!(formatter, "{command} only supported for web core")
            }
            Self::MissingPackage { command } => write!(formatter, "Usage: mgc {command} <package>"),
            Self::LinkExists { name } => write!(formatter, "{name} already exists in node_modules"),
            Self::UnlinkNotLinked { name } => {
                write!(formatter, "{name} is not linked in node_modules")
            }
            Self::NotPackageLink { name } => write!(
                formatter,
                "refusing to remove non-link package entry '{name}' from node_modules"
            ),
            Self::InvalidLinkName { package } => {
                write!(formatter, "invalid package link target '{package}'")
            }
            Self::UnsafeLinkParent { package } => write!(
                formatter,
                "refusing to access package link '{package}' through a symlink or non-directory scope"
            ),
            Self::LocalPathNotFound { path } => {
                write!(
                    formatter,
                    "local package path not found: {}",
                    path.display()
                )
            }
            Self::PackageNotFound { package } => write!(
                formatter,
                concat!(
                    "package '{package}' not found in local node_modules or MagiCore global ",
                    "package roots.\nUse 'mgc install' to install it first, provide a ",
                    "local path, or configure MAGICORE_GLOBAL_PACKAGE_ROOT."
                ),
                package = package
            ),
            Self::Filesystem { source } => fmt::Display::fmt(source, formatter),
        }
    }
}

impl StdError for WebLinkError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Filesystem { source } => Some(source),
            Self::UnsupportedCore { .. }
            | Self::MissingPackage { .. }
            | Self::LinkExists { .. }
            | Self::UnlinkNotLinked { .. }
            | Self::NotPackageLink { .. }
            | Self::InvalidLinkName { .. }
            | Self::UnsafeLinkParent { .. }
            | Self::LocalPathNotFound { .. }
            | Self::PackageNotFound { .. } => None,
        }
    }
}
