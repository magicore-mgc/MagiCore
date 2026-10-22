//! Shared core-command module wiring.
//! Wiring module cho dịch vụ lệnh core dùng chung.

pub mod container_flags;

#[path = "shared.rs"]
mod implementation;

pub(crate) use implementation::*;
