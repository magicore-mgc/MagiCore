//! Typed compiler diagnostics — chẩn đoán compiler có kiểu.
use std::fmt;

#[derive(Debug)]
pub(in crate::bundler::dev_server::implementation) struct CompilerDiagnostics(String);

impl CompilerDiagnostics {
    pub(super) fn new(message: String) -> Self {
        Self(message)
    }
}

impl fmt::Display for CompilerDiagnostics {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for CompilerDiagnostics {}
