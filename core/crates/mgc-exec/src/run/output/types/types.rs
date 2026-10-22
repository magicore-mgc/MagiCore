//! Shared capture mode and completed process output record.
//! Chế độ capture dùng chung và bản ghi output process đã hoàn tất.

use std::process::ExitStatus;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::run::implementation) enum OutputMode {
    Capture,
    Inherit,
}

pub(in crate::run::implementation) struct ExecOutcome {
    pub(in crate::run::implementation) status: ExitStatus,
    pub(in crate::run::implementation) stdout: Vec<u8>,
    pub(in crate::run::implementation) stderr: Vec<u8>,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
