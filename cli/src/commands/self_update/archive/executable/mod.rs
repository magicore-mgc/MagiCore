//! Export platform permission setup.
//! Công bố cấu hình quyền chạy theo nền tảng.

mod executable;
pub(in crate::commands::self_update) use executable::set_executable;
