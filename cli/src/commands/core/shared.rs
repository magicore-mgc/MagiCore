//! Shared core-command service facade.
//! Facade dịch vụ lệnh dùng chung cho các core.

#[path = "shared/mod.rs"]
mod implementation;

pub(crate) use implementation::*;
