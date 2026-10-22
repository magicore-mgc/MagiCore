//! Dispatch type contracts — stable exports for the CLI routing layer.
//! Hợp đồng kiểu dispatch — giữ namespace ổn định cho tầng định tuyến CLI.
mod common_command;
mod core_command;
mod detect_ecosystem;
mod dispatch_command;

pub use common_command::CommonCommand;
pub use core_command::CoreCommand;
pub use detect_ecosystem::detect_ecosystem;
pub use dispatch_command::DispatchCommand;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
