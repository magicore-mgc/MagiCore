//! Runtime invocation facade — export runtime argv parsers.
//! Facade invocation runtime — export parser argv theo runtime.

#[path = "runtime_invocation.rs"]
mod implementation;
pub(super) use implementation::{
    argv_text, forbidden_entry_name, process_invocation_program, runtime_invocation,
    windows_cmd_invocation,
};
