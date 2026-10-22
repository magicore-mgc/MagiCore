//! Wiring for executable-bit inspection and updates.
//! Wiring cho kiểm tra và cập nhật quyền executable.

mod executable;
pub use executable::{is_executable, set_executable};
