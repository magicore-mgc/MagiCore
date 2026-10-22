//! Compiler owner wiring — nối owner compiler.
#[path = "error.rs"]
mod error;
#[path = "compiler.rs"]
mod implementation;
pub(super) use self::error::CompilerDiagnostics;
pub(super) use self::implementation::{compile_error_body, compile_transpiled_source};
