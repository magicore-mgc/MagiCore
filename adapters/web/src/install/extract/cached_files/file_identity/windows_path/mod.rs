//! Windows path helpers owner.
//! Owner helper path Windows.

mod windows_path;

pub(super) use windows_path::{
    windows_cached_path_is_safe, windows_final_path, windows_opened_path_is_within,
};
