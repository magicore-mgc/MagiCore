//! Append captured command output owner wiring.
//! File này chỉ nối helper gắn output command vào response.

mod append_captured_stdout;
pub(in crate::commands::mcp) use append_captured_stdout::append_captured_stdout;
