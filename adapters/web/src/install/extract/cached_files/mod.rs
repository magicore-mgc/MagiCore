//! Secure package-cache file snapshot and identity owner.
//! Module chủ quản snapshot file cache package an toàn và identity.
mod cached_files;
mod file_identity;
mod types;
pub(crate) use cached_files::{cached_archive_file_matches, cached_package_files};
#[cfg(test)]
pub(crate) use file_identity::cached_file_identity;
#[cfg(windows)]
pub(crate) use file_identity::windows_opened_path_is_within;
