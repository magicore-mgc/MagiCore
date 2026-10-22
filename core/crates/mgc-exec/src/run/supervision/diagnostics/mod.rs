//! Supervision diagnostics facade.
//! Facade chẩn đoán supervision.

#[path = "error.rs"]
mod error;
#[path = "diagnostics.rs"]
mod implementation;

pub(in crate::run::implementation) use implementation::{forbidden_child_error, timeout_error};
