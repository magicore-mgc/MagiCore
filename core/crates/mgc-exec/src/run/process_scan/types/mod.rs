//! Shared process-scan data types.
//! Kiểu dữ liệu dùng chung cho process scan.

#[cfg(any(unix, test))]
#[path = "types.rs"]
mod implementation;

#[cfg(any(unix, test))]
pub(in crate::run::implementation) use implementation::ProcessScanSnapshot;
#[cfg(unix)]
pub(in crate::run::implementation) use implementation::RootExitScan;
