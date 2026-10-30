//! Windows process scan facade — export identity scanning and retry.
//! Facade quét process Windows — export quét identity và retry.

#[path = "windows.rs"]
pub(in crate::run::implementation) mod implementation;
// The retry helper is consumed one level up (the Windows job inspection
// re-export), so the facade forwards it at the same scope its item declares.
// (Helper retry được consumer ở một cấp trên, nên facade chuyển tiếp ở đúng
// scope mà item khai báo.)
pub(in crate::run::implementation) use implementation::{
    WindowsProcessScan, WindowsProcessSnapshot,
};
// The scan-result module imports the retry helper through this facade, so the
// facade must forward it (E0432 until it did).
// (Module scan-result import helper retry qua facade này, nên facade phải
// chuyển tiếp — trước đó là E0432.)
#[cfg(windows)]
pub(in crate::run::implementation) use implementation::retry_windows_process_scan;
