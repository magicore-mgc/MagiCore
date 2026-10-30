//! Windows process scan facade — export identity scanning and retry.
//! Facade quét process Windows — export quét identity và retry.

#[path = "windows.rs"]
pub(in crate::run::implementation) mod implementation;

// Only the snapshot tests name the ancestry type; `windows.rs` already forwards
// it for them, so the facade does not duplicate the import.
// (Chỉ test snapshot gọi type ancestry; `windows.rs` đã chuyển tiếp cho chúng nên
// facade không lặp import.)
#[cfg(windows)]
pub(in crate::run::implementation) use implementation::retry_windows_process_scan;
pub(in crate::run::implementation) use implementation::{
    WindowsProcessScan, WindowsProcessSnapshot,
};
