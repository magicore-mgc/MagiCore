// Bare update diagnostics — reject unrelated commands routed into this mapper.
// Diagnostic update bare — từ chối lệnh khác bị chuyển nhầm vào mapper.
use std::fmt;

#[derive(Debug)]
pub(super) enum BareUpdateError {
    UnexpectedCommand,
}

impl fmt::Display for BareUpdateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedCommand => {
                formatter.write_str("bare update mapper received a different verb")
            }
        }
    }
}

impl std::error::Error for BareUpdateError {}
