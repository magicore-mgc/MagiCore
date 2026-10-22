//! Windows user PATH module wiring and local test registration.
//! Khai báo module PATH Windows và đăng ký test cục bộ.

mod windows;

pub(super) use windows::{USER_PATH_SCRIPT, ensure_windows_user_path};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
