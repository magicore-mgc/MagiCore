//! Shared atomic publication owner.
//! Module chủ quản ghi file atomic dùng chung.
mod atomic_write_project_file;
pub(crate) use atomic_write_project_file::atomic_write_project_file;
