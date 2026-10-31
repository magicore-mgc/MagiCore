//! Shared process-scan data types.
//! Kiểu dữ liệu dùng chung cho process scan.

// RootExitScan is part of the shared supervision result type, so this inner
// module must compile on every platform; only ProcessScanSnapshot (Unix/Linux
// fields) and the Unix scan stay gated.
// (RootExitScan thuộc type kết quả supervision dùng chung nên module này phải
// biên dịch mọi nền tảng; chỉ ProcessScanSnapshot và scan Unix giữ gate.)
#[path = "types.rs"]
mod implementation;

#[cfg(any(unix, test))]
pub(in crate::run::implementation) use implementation::ProcessScanSnapshot;
pub(in crate::run::implementation) use implementation::RootExitScan;
