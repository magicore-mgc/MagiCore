//! Child command environment facade.
//! Facade environment của child command.

#[path = "command_environment.rs"]
mod implementation;

pub(super) use implementation::configure_command_environment;
