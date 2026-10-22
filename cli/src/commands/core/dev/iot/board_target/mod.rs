mod board_target;
pub(in crate::commands::core::dev) use board_target::{chip, resolve_target};

#[cfg(test)]
#[path = "test/board_target.rs"]
mod tests;
