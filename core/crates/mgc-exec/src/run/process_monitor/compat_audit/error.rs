//! Typed errors for compatibility-process audit recording.
//! Lỗi typed khi ghi audit process compatibility.

use std::fmt;

#[derive(Debug)]
pub(in crate::run::implementation) enum CompatAuditError {
    DedupStatePoisoned,
    WriteFailed(anyhow::Error),
}

impl fmt::Display for CompatAuditError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::DedupStatePoisoned => "compat audit process set was poisoned",
            Self::WriteFailed(_) => "compat audit event could not be written",
        })
    }
}

impl std::error::Error for CompatAuditError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DedupStatePoisoned => None,
            Self::WriteFailed(source) => Some(source.as_ref()),
        }
    }
}

#[cfg(test)]
#[path = "test/error.rs"]
mod test;
