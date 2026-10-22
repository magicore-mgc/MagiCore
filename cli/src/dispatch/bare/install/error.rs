// Bare install diagnostics — report an invalid command routed to this mapper.
// Diagnostic install bare — báo lỗi khi mapper nhận nhầm nhóm lệnh.
use std::fmt;

#[derive(Debug)]
pub(super) enum BareInstallError {
    UnexpectedCommand,
}

impl fmt::Display for BareInstallError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedCommand => {
                formatter.write_str("bare install mapper received a different verb")
            }
        }
    }
}

impl std::error::Error for BareInstallError {}
