use std::error::Error as StdError;
use std::fmt::{Display, Formatter};

/// Failure categories owned by the JSONL append boundary.
#[derive(Debug)]
pub(in crate::audit::serialized_append) enum AuditAppendError {
    LockPoisoned,
}

impl Display for AuditAppendError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LockPoisoned => formatter.write_str("audit append lock was poisoned"),
        }
    }
}

impl StdError for AuditAppendError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::LockPoisoned => None,
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
