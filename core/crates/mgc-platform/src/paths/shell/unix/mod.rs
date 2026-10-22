//! Unix shell PATH module wiring and stable local entry export.
//! Khai báo module PATH Unix và export entry nội bộ ổn định.

mod atomic_replace_profile;
mod ensure_directory_chain;
mod ensure_profile_entry;
mod managed_path_profile_block;
mod unix;

pub(super) use unix::ensure_bin_on_path;
