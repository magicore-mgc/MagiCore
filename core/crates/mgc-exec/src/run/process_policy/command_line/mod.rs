//! Legacy text command-line matcher exports.
//! Export matcher command-line dạng text hiện hữu.

#[path = "command_line.rs"]
mod implementation;
#[cfg(test)]
pub(in crate::run::implementation) use implementation::forbidden_process_match;
pub(in crate::run::implementation) use implementation::forbidden_process_name;
