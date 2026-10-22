//! CoreCommand owner exports — stores per-core lifecycle payloads and policy controls.
//! Module xuất payload vòng đời và policy riêng cho từng core.
mod core_command;
pub use core_command::CoreCommand;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
