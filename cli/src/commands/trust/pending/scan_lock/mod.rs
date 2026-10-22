//! Scan-lock owner wiring.
//! Wiring owner khóa khi quét.

mod scan_lock;
pub(super) use self::scan_lock::acquire_scan_lock;
