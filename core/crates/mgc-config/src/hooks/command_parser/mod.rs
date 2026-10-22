//! Hook command parser exports.
//! Export parser command hook.

mod command_parser;

pub(super) use command_parser::parse_hook_command;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
