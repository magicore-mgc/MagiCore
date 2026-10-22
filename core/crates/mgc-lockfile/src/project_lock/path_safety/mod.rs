//! Wires project path safety, local errors, and Windows reparse inspection.
//! Nối an toàn path project, lỗi cục bộ và kiểm tra reparse trên Windows.

mod error;
mod path_safety;
#[cfg(windows)]
mod windows_reparse;

pub use self::path_safety::path_is_link_or_reparse;
pub(in crate::project_lock) use self::path_safety::{ensure_real_dir, refuse_link_swap};
