//! Typed failures raised while owning a Windows Job Object and its process handles.
//! Lỗi typed khi quản lý Windows Job Object và các process handle thuộc owner này.

use std::{fmt, num::TryFromIntError};

/// Errors from Windows APIs retain the failing operation and native error code.
/// Lỗi API Windows giữ tên thao tác và mã lỗi native để truy nguyên chính xác.
#[derive(Debug)]
pub(in crate::run::implementation) struct WindowsApiFailure {
    operation: &'static str,
    code: u32,
}

impl WindowsApiFailure {
    pub(super) fn new(operation: &'static str, code: u32) -> Self {
        Self { operation, code }
    }
}

impl fmt::Display for WindowsApiFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Windows API {} failed with error code {}",
            self.operation, self.code
        )
    }
}

impl std::error::Error for WindowsApiFailure {}

/// Failures classified at the Windows Job Object owner boundary.
/// Lỗi được phân loại ngay tại ranh giới owner Windows Job Object.
#[derive(Debug)]
pub(in crate::run::implementation) enum WindowsJobError {
    NativeApi(WindowsApiFailure),
    ProcessIdBufferTooLarge(TryFromIntError),
    ImagePathLengthConversion(TryFromIntError),
    InvalidImagePathLength {
        length: u32,
        capacity: usize,
    },
    InvalidProcessIdCount {
        listed: usize,
        capacity: usize,
    },
    ProcessIdCapacityExceeded {
        assigned: usize,
        maximum: usize,
    },
    ProcessIdCapacityLimit {
        maximum: usize,
    },
    ProcessIdConversion(TryFromIntError),
    IncompleteProcessSnapshot {
        assigned: usize,
        listed: usize,
        capacity: usize,
    },
    MissingPrimaryThread {
        process_id: u32,
    },
    UnexpectedSuspendCount {
        process_id: u32,
        count: u32,
    },
    ActiveMemberUnavailable {
        process_id: u32,
        source: WindowsApiFailure,
    },
    MembershipQueryFailed {
        process_id: u32,
        source: WindowsApiFailure,
    },
    MembershipChanged {
        process_id: u32,
    },
    HandleCloseFailures {
        failures: Vec<WindowsApiFailure>,
        omitted_count: usize,
    },
}

impl WindowsJobError {
    pub(super) fn native_api(operation: &'static str, code: u32) -> Self {
        Self::NativeApi(WindowsApiFailure::new(operation, code))
    }

    pub(super) fn handle_close_failures(
        failures: Vec<WindowsApiFailure>,
        omitted_count: usize,
    ) -> Self {
        Self::HandleCloseFailures {
            failures,
            omitted_count,
        }
    }
}

impl fmt::Display for WindowsJobError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NativeApi(source) => source.fmt(formatter),
            Self::ProcessIdBufferTooLarge(_) => {
                formatter.write_str("Windows job process ID buffer exceeds the API size limit")
            }
            Self::ImagePathLengthConversion(_) => {
                formatter.write_str("Windows returned an invalid process image path length")
            }
            Self::InvalidImagePathLength { length, capacity } => write!(
                formatter,
                "Windows returned an invalid process image path length {length} (capacity {capacity})"
            ),
            Self::InvalidProcessIdCount { listed, capacity } => write!(
                formatter,
                "Windows job returned {listed} process IDs for a buffer capacity of {capacity}"
            ),
            Self::ProcessIdCapacityExceeded { assigned, maximum } => write!(
                formatter,
                "Windows job has {assigned} process IDs, exceeding the configured capacity {maximum}"
            ),
            Self::ProcessIdCapacityLimit { maximum } => write!(
                formatter,
                "Windows job process ID snapshot exceeds the configured capacity {maximum}"
            ),
            Self::ProcessIdConversion(_) => {
                formatter.write_str("Windows job returned a process ID outside the supported range")
            }
            Self::IncompleteProcessSnapshot {
                assigned,
                listed,
                capacity,
            } => write!(
                formatter,
                "Windows job process ID snapshot remained incomplete after bounded retries (assigned={assigned}, listed={listed}, capacity={capacity})"
            ),
            Self::MissingPrimaryThread { process_id } => write!(
                formatter,
                "suspended Windows child PID {process_id} has no discoverable primary thread"
            ),
            Self::UnexpectedSuspendCount { process_id, count } => write!(
                formatter,
                "suspended Windows child PID {process_id} had an unexpected suspend count: {count}"
            ),
            Self::ActiveMemberUnavailable { process_id, source } => write!(
                formatter,
                "cannot capture active Windows job child PID {process_id} before identity verification: {source}"
            ),
            Self::MembershipQueryFailed { process_id, source } => write!(
                formatter,
                "failed to verify Windows job child PID {process_id} membership: {source}"
            ),
            Self::MembershipChanged { process_id } => write!(
                formatter,
                "Windows job child PID {process_id} changed membership before identity verification"
            ),
            Self::HandleCloseFailures {
                failures,
                omitted_count,
            } => {
                let total_count = failures.len().saturating_add(*omitted_count);
                write!(formatter, "failed to close {total_count} Windows handle(s)")?;
                for failure in failures {
                    write!(formatter, "; {failure}")?;
                }
                if *omitted_count > 0 {
                    write!(formatter, "; {omitted_count} additional failure(s) omitted")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for WindowsJobError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::NativeApi(source)
            | Self::ActiveMemberUnavailable { source, .. }
            | Self::MembershipQueryFailed { source, .. } => Some(source),
            Self::ProcessIdBufferTooLarge(source)
            | Self::ImagePathLengthConversion(source)
            | Self::ProcessIdConversion(source) => Some(source),
            Self::InvalidImagePathLength { .. }
            | Self::InvalidProcessIdCount { .. }
            | Self::ProcessIdCapacityExceeded { .. }
            | Self::ProcessIdCapacityLimit { .. }
            | Self::IncompleteProcessSnapshot { .. }
            | Self::MissingPrimaryThread { .. }
            | Self::UnexpectedSuspendCount { .. }
            | Self::MembershipChanged { .. } => None,
            Self::HandleCloseFailures { failures, .. } => failures
                .first()
                .map(|failure| failure as &(dyn std::error::Error + 'static)),
        }
    }
}

#[cfg(test)]
#[path = "test/error.rs"]
mod test;
