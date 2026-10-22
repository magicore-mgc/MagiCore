// Bare route exports — expose the coordinator through the existing dispatch path.
// Module này xuất coordinator qua đường dẫn dispatch hiện có.
mod add;
mod bare;
mod ecosystem;
mod error;
mod install;
mod list;
mod remove;
mod update;
mod validate;

pub use bare::bare_core_command;
