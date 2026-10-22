//! Wires Windows reparse-point inspection and platform tests.
//! Nối kiểm tra reparse point Windows và test riêng platform.

mod windows_reparse;
pub(in crate::project_lock::path_safety) use self::windows_reparse::windows_reparse_point;
