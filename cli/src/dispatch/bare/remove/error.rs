// Bare remove diagnostics — reject unrelated commands routed to this mapper.
// Diagnostic remove bare — từ chối lệnh khác bị chuyển nhầm vào mapper.
use std::fmt;

#[derive(Debug)]
pub(super) enum BareRemoveError {
    UnexpectedCommand,
}

impl fmt::Display for BareRemoveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedCommand => {
                formatter.write_str("bare remove mapper received a different verb")
            }
        }
    }
}

impl std::error::Error for BareRemoveError {}
