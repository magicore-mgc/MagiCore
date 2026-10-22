//! Audit strictness and exit policy facade.
//! Facade strictness và exit policy audit.

#[path = "strict_mode.rs"]
mod strict_mode;

pub(crate) use strict_mode::{StrictMode, enforce_audit_exit};
