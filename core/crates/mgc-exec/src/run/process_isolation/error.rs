//! Typed process-isolation failures at the pre-spawn security boundary.
//! Lỗi typed của cô lập process tại ranh giới bảo mật trước spawn.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::run::implementation) struct ProcessInspectionUnavailable;

impl fmt::Display for ProcessInspectionUnavailable {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(
            "native process-tree inspection is unavailable on this platform; refusing to treat the process tree as clean",
        )
    }
}

impl std::error::Error for ProcessInspectionUnavailable {}
