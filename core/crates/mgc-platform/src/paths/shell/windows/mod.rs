//! Windows user PATH module wiring and local test registration.
//! Khai báo module PATH Windows và đăng ký test cục bộ.

// `shell.rs` (one level up) calls the installer and the tests read the script
// constant, so the inner module and its items share that scope.
// (`shell.rs` cao hơn một mức gọi installer và test đọc hằng script, nên
// module bên trong và các item của nó cùng scope đó.)
pub(super) mod windows;

pub(super) use windows::{USER_PATH_SCRIPT, ensure_windows_user_path};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
