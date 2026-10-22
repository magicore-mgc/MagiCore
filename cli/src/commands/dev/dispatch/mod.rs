//! Core route selection for the shared dev command.
//! Chọn route core cho lệnh dev dùng chung.

mod dispatch;

pub(super) use dispatch::dispatch;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
