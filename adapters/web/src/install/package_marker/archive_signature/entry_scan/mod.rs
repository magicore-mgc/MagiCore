//! Validated file-entry scanning for gzip/tar archives.
//! Quét entry file đã xác minh của archive gzip/tar.

mod entry_scan;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

pub(in crate::install::package_marker::archive_signature) use entry_scan::{
    TarballFileSignature, read_tarball_files,
};
