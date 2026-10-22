//! Compatibility audit writer facade.
//! Facade ghi audit compatibility.

#[path = "error.rs"]
mod error;
#[path = "compat_audit.rs"]
mod implementation;

pub(in crate::run::implementation) use implementation::audit_compat_process;
