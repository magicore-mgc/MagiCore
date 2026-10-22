//! Shared process-policy data types.
//! Kiểu dữ liệu process-policy dùng chung.

#[path = "types.rs"]
mod implementation;

pub(in crate::run::implementation) use implementation::ForbiddenProcess;
