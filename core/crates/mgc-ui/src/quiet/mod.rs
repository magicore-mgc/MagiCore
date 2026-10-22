//! Quiet-state wiring — wiring cho trạng thái quiet.
//! The state owner is shared by output modules without duplicating policy.
//! Các module output dùng chung state, không tạo bản sao policy.

mod quiet;
#[cfg(test)]
#[path = "test/quiet.rs"]
mod tests;

pub use quiet::{is_quiet, set_quiet};

#[cfg(test)]
pub(crate) use quiet::test_lock;
