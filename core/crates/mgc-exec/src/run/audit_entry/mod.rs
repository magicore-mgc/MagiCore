//! Execution-report to audit-entry facade.
//! Facade chuyển report thực thi thành audit entry.

#[path = "audit_entry.rs"]
mod implementation;

pub(in crate::run) use implementation::entry_from;
