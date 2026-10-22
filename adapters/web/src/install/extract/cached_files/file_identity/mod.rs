//! Platform file identity owner.
//! Module chủ quản định danh file theo OS.
mod file_identity;
pub(crate) use file_identity::cached_file_identity;
pub(super) use file_identity::open_cached_file_no_follow;
#[cfg(windows)]
pub(crate) use file_identity::windows_opened_path_is_within;
