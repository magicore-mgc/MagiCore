//! DispatchCommand owner exports — keeps common and per-core routes distinct.
//! Module xuất DispatchCommand, giữ riêng tuyến dùng chung và tuyến theo core.
mod dispatch_command;
pub use dispatch_command::DispatchCommand;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
