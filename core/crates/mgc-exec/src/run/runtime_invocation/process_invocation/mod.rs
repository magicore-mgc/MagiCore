//! Process-entrypoint normalization exports.
//! Export chuẩn hóa process entrypoint.

#[path = "process_invocation.rs"]
mod implementation;
pub(in crate::run::implementation) use implementation::process_invocation_program;
