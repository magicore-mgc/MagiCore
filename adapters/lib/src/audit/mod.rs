//! Stable Lib audit routes and module wiring.
//! Route audit ổn định và wiring module Lib.

pub mod scanner;

pub use mgc_audit::scanners::{audit_go, audit_python, audit_rust};

mod aggregate;
mod audit;
mod discovery;

pub(crate) use audit::run_audit;
