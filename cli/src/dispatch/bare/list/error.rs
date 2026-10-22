// Bare list diagnostics — reject unrelated commands routed to this mapper.
// Diagnostic list bare — từ chối lệnh khác bị chuyển nhầm vào mapper.
use std::fmt;

#[derive(Debug)]
pub(super) enum BareListError {
    UnexpectedCommand,
}

impl fmt::Display for BareListError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedCommand => {
                formatter.write_str("bare list mapper received a different verb")
            }
        }
    }
}

impl std::error::Error for BareListError {}
