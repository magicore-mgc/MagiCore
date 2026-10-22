// Bare add diagnostics — reject unrelated commands routed to this mapper.
// Diagnostic add bare — từ chối lệnh khác bị chuyển nhầm vào mapper.
use std::fmt;

#[derive(Debug)]
pub(super) enum BareAddError {
    UnexpectedCommand,
}

impl fmt::Display for BareAddError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedCommand => {
                formatter.write_str("bare add mapper received a different verb")
            }
        }
    }
}

impl std::error::Error for BareAddError {}
