//! Keep the in-crate Windows process snapshot and scan result shapes.
//! Giữ type snapshot process Windows và kết quả scan dùng nội bộ.

use crate::run::implementation::ForbiddenProcess;
use std::ffi::OsString;

#[cfg(any(windows, test))]
pub(in crate::run::implementation) struct WindowsProcessSnapshot {
    pub(in crate::run::implementation) pid: u32,
    #[cfg(test)]
    pub(in crate::run::implementation) parent_pid: Option<u32>,
    #[cfg(test)]
    pub(in crate::run::implementation) creation_time: Option<u64>,
    pub(in crate::run::implementation) image_name: String,
    pub(in crate::run::implementation) command: Vec<OsString>,
}

#[cfg(any(windows, test))]
pub(in crate::run::implementation) enum WindowsProcessScan {
    Clean,
    Forbidden(ForbiddenProcess),
    MissingCommandLine {
        pid: u32,
        image_name: String,
    },
    #[cfg(test)]
    MissingProcessIdentity {
        pid: u32,
    },
    #[cfg(test)]
    UnverifiableAncestry {
        parent_pid: u32,
        child_pid: u32,
    },
    ChildExited,
    DeadlineReached,
}
