mod board_target;
pub(crate) mod dev_command;
mod firmware_artifact;
mod iot;
mod tool_execution;

pub(super) use board_target::{chip, resolve_target};
pub(super) use firmware_artifact::find_elf;
pub use iot::flash;
pub(super) use tool_execution::run_tool;
