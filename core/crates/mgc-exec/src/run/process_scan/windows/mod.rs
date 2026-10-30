//! Windows process scan facade — export identity scanning and retry.
//! Facade quét process Windows — export quét identity và retry.

#[path = "windows.rs"]
mod implementation;
// The retry helper is consumed one level up (the Windows job inspection
// re-export), so the facade forwards it at the same scope its item declares.
// (Helper retry được consumer ở một cấp trên, nên facade chuyển tiếp ở đúng
// scope mà item khai báo.)
pub(in crate::run::implementation) use implementation::{
    WindowsProcessScan, WindowsProcessSnapshot,
};
// `retry_windows_process_scan` is reached through the inner implementation
// module by its consumers, so forwarding it here would be an unused import on
// non-Windows builds.
// (`retry_windows_process_scan` được consumer lấy qua module implementation
// bên trong, nên chuyển tiếp ở đây là import thừa trên build không-Windows.)
